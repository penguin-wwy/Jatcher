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
extern void config_init();

extern void preprocess_method(Klass_Method *);

extern void *get_var_name(jmethodID, u_int32_t, size_t *);

extern const char *get_output_file();

extern void deallocate_str_vec_buffer(const char **, size_t);

#endif //JATCHER_BRIDGE_H
