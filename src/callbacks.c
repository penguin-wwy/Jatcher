//
// Created by penguin on 19-12-28.
//

#include "tools.h"
#include "callbacks.h"
#include "error_info.h"

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

void JNICALL callbackEventBreakpoint(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread, jmethodID method, jlocation location) {

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