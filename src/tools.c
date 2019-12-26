//
// Created by penguin on 19-12-25.
//

#include <jni.h>
#include <jvmti.h>

#include "tools.h"

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
	jvmtiEnv *jvmti = NULL;
	if (JNI_OK != (*vm)->GetEnv(vm, (void **) (&jvmti), JVMTI_VERSION_1_2)) {
		ERROT_EXIT(JVMTI_ERR, "ERROR: Unable to access JVMTI.\n");
	}

	INFO_LOG("Parse config...");
	if (0 != parse_config(options)) {

	}

	jvmtiCapabilities capabilities;
	memset(&capabilities, 0, sizeof(jvmtiCapabilities));
	capabilities.can_access_local_variables = 1;
	capabilities.can_generate_breakpoint_events = 1;
	capabilities.can_generate_exception_events = 1;
	capabilities.can_get_bytecodes = 1;
	capabilities.can_get_constant_pool = 1;
	capabilities.can_get_line_numbers = 1;


}