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
//	char *class_signature = NULL;
//	if (checkJVMTIError(jvmti_env, (*jvmti_env)->GetClassSignature(jvmti_env, klass, &class_signature, NULL), CLASS_NAME_FAIL)) {
//		return;
//	}


}