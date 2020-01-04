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

extern const char **get_var_name(jmethodID method, u_int32_t line, size_t *len);

extern const char *get_output_file();

#endif //JATCHER_BRIDGE_H
