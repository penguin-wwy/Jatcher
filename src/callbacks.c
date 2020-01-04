//
// Created by penguin on 19-12-28.
//

#include "tools.h"
#include "callbacks.h"
#include "error_info.h"

static FILE *file_stream;

#define PRINT_LINE(LINE)									\
	do {													\
		char line_buf[32] = {0};							\
		sprintf(line_buf, "{\"line\": %u}, ", LINE);		\
		fwrite(line_buf, 1, strlen(line_buf), file_stream);	\
	} while (0)

#define PRINT(STR)									\
	do {											\
		fwrite(STR, 1, strlen(STR), file_stream);	\
	} while (0)

void JNICALL callbackVMInit(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread) {
	const char *file_path = get_output_file();
	if (file_path == NULL) {
		file_stream = stdout;
	} else {
		file_stream = fopen(file_path, "w");
		if (file_stream == NULL) ERROT_EXIT(FILE_ERR, "Result file %s open failed", file_path);
	}
}

void JNICALL callbackVMDeath(jvmtiEnv *jvmti_env, JNIEnv* jni_env) {
	if (file_stream != stdout) {
		fclose(file_stream);
	}
}

void set_break_point(Klass_Method *km, const u_int32_t *lines, size_t len) {
	jvmtiEnv *jvmti = km->jvmti;
	jmethodID jmethod = km->method_id;

	INFO_LOG("Set break point in %s.%s\n", km->class_signature, km->method_name);

	jint line_entry_count = 0;
	jvmtiLineNumberEntry *line_number_entry = NULL;
	if (checkJVMTIError(jvmti, (*jvmti)->GetLineNumberTable(jvmti, jmethod, &line_entry_count, &line_number_entry), LINE_NUMBER_FAIL)) {
		return;
	}

	for (size_t i = 0; i < len; i++) {
		u_int32_t line = lines[i];
		jint loc = 0;
		while (loc < line_entry_count && line_number_entry[loc].line_number != line) {
			loc++;
		}
		if (loc < line_entry_count) {
			checkJVMTIError(jvmti, (*jvmti)->SetBreakpoint(jvmti, jmethod, line_number_entry[loc].start_location), "Set Breakpoint fail");
		}
	}
}

void JNICALL callbackClassPrepare(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread, jclass klass) {
	char *class_signature = NULL;
	if (checkJVMTIError(jvmti_env, (*jvmti_env)->GetClassSignature(jvmti_env, klass, &class_signature, NULL), CLASS_NAME_FAIL)) {
		return;
	}

	jint method_count;
	jmethodID *method_array;
	if (checkJVMTIError(jvmti_env, (*jvmti_env)->GetClassMethods(jvmti_env, klass, &method_count, &method_array), CLASS_METHOD_FAIL)) {
		return;
	}

	for (int i = 0; i < method_count; i++) {
		char *method_name = NULL;
		char *method_signature = NULL;
		if (checkJVMTIError(jvmti_env, (*jvmti_env)->GetMethodName(jvmti_env, method_array[i], &method_name, &method_signature, NULL), METHOD_NAME_FAIL)) {
			continue;
		}
		if (method_name != NULL && method_signature != NULL) {
			Klass_Method km = {jvmti_env, jni_env, klass, class_signature, method_array[i], method_name, method_signature};
			preprocess_method(&km);
		}
	}
}

jint get_local_variable(jvmtiEnv *jvmti, jmethodID jmethod, const char *name, jlocation loc, char **signature) {
	jint count = 0;
	jvmtiLocalVariableEntry *loc_var_entry = NULL;
	if (checkJVMTIError(jvmti, (*jvmti)->GetLocalVariableTable(jvmti, jmethod, &count, &loc_var_entry), LOCATION_VAR_FAIL)) {
		return -1;
	}
	for (jint i = 0; i < count; i++) {
		if (loc_var_entry[i].start_location == loc && strcmp(loc_var_entry[i].name, name) == 0) {
			*signature = loc_var_entry[i].signature;
			return loc_var_entry[i].slot;
		}
	}
	return -1;
}

void print_variable(jvmtiEnv *jvmti, JNIEnv* jni_env, jthread thread, jint slot, const char *name, const char *signature) {
	char string[64] = {0};
#define PRINT_FAIL()													\
	do {																\
		sprintf(string, "\"%s:%s\": \"get failed\"", name, signature);	\
		PRINT(string);													\
	} while (0)

	switch (*signature) {
		case 'Z':
		case 'B':
		case 'C':
		case 'S':
		case 'I': {
			jint value = 0;
			if (checkJVMTIError(jvmti, (*jvmti)->GetLocalInt(jvmti, thread, 0, slot, &value), GET_LOCAL_VALUE_FAIL)) {
				sprintf(string, "{\"%s:%s\": %d}", name, signature, value);
				PRINT(string);
			} else {
				PRINT_FAIL();
			}
			break;
		}
		case 'J': { // long
			jlong value = 0;
			if (checkJVMTIError(jvmti, (*jvmti)->GetLocalLong(jvmti, thread, 0, slot, &value), GET_LOCAL_VALUE_FAIL)) {
				sprintf(string, "{\"%s:%s\": %ld}", name, signature, value);
				PRINT(string);
			} else {
				PRINT_FAIL();
			}
			break;
		}
		case 'F': { // float
			jfloat value = 0;
			if (checkJVMTIError(jvmti, (*jvmti)->GetLocalFloat(jvmti, thread, 0, slot, &value), GET_LOCAL_VALUE_FAIL)) {
				sprintf(string, "{\"%s:%s\": %f}", name, signature, value);
				PRINT(string);
			} else {
				PRINT_FAIL();
			}
			break;
		}
		case 'D': { // double
			jdouble value = 0;
			if (checkJVMTIError(jvmti, (*jvmti)->GetLocalDouble(jvmti, thread, 0, slot, &value), GET_LOCAL_VALUE_FAIL)) {
				sprintf(string, "{\"%s:%s\": %lf}", name, signature, value);
				PRINT(string);
			} else {
				PRINT_FAIL();
			}
			break;
		}
		default: {
			jobject value = NULL;
			if (checkJVMTIError(jvmti, (*jvmti)->GetLocalObject(jvmti, thread, 0, slot, &value), GET_LOCAL_VALUE_FAIL)) {

			} else {
				PRINT_FAIL();
			}
		}
	}
}

void JNICALL callbackEventBreakpoint(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread, jmethodID method, jlocation location) {
	char *method_name = NULL;
	char *method_signature = NULL;
	if (checkJVMTIError(jvmti_env, (*jvmti_env)->GetMethodName(jvmti_env, method, &method_name, &method_signature, NULL), METHOD_NAME_FAIL)) {
		return;
	}

	jvmtiLineNumberEntry *line_number_entry = NULL;
	jint line_entry_count = 0;
	if (checkJVMTIError(jvmti_env, (*jvmti_env)->GetLineNumberTable(jvmti_env, method, &line_entry_count, &line_number_entry), LINE_NUMBER_FAIL)) {
		return;
	}
	jint line_index = 0;
	while (line_index < line_entry_count && line_number_entry[line_index].start_location != location) {
		line_index++;
	}
	if (line_index == line_entry_count) {
		return;
	}
	jvmtiLineNumberEntry line_entry = line_number_entry[line_index];
	size_t len = 0;
	const char **var_list = get_var_name(method, line_entry.line_number, &len);
	for (u_int32_t i = 0; i < len; i++) {
		char *signature = NULL;
		jint slot = get_local_variable(jvmti_env, method, var_list[i], location, signature);
		PRINT_LINE(line_entry.line_number);
		print_variable(jvmti_env, jni_env, thread, slot, var_list[i], signature);
	}
}