//
// Created by penguin on 19-12-28.
//

#include "tools.h"
#include "callbacks.h"
#include "error_info.h"

//void set_break_point(jvmtiEnv *jvmti, JNIEnv *jniEnv, jclass klass, ) {
//
//}

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