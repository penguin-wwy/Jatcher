//
// Created by penguin on 19-12-25.
//

#ifndef JATCHER_BRIDGE_H
#define JATCHER_BRIDGE_H

#include <jni.h>
#include <jvmti.h>

typedef struct {
	jvmtiEnv *jvmti;
	JNIEnv *jni;
	jclass class_id;
	const char *class_signature;
	jmethodID method_id;
	const char *method_name;
	const char *method_signature;
} Klass_Method;

extern void info_log(const char *);
extern void error_log(const char *);
extern void warn_log(const char *);

extern int32_t parse_config(const char *);

extern void preprocess_method(Klass_Method *);

#endif //JATCHER_BRIDGE_H
