//
// Created by penguin on 19-12-28.
//

#ifndef JATCHER_CALLBACKS_H
#define JATCHER_CALLBACKS_H

#include <jvmti.h>

void JNICALL callbackVMInit(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread);
void JNICALL callbackVMDeath(jvmtiEnv *jvmti_env, JNIEnv* jni_env);
void JNICALL callbackClassPrepare(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread, jclass klass);
void JNICALL callbackEventBreakpoint(jvmtiEnv *jvmti_env, JNIEnv* jni_env, jthread thread, jmethodID method, jlocation location);

#endif //JATCHER_CALLBACKS_H
