//
// Created by penguin on 19-12-25.
//

#include "tools.h"
#include "callbacks.h"
#include "error_info.h"

#define false 0
#define true 1
#define bool int

bool checkJVMTIError(jvmtiEnv *jvmti, jvmtiError err_code, const char *str) {
	if (err_code != JVMTI_ERROR_NONE) {
		char *err_str = NULL;
		(*jvmti)->GetErrorName(jvmti, err_code, &err_str);
		WARN_LOG("JVMTI WARN: %d %s: %s\n", err_code, err_str == NULL ? "Unknown error" : err_str, str == NULL ? "" : str);
		return true;
	}
	return false;
}

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
	jvmtiEnv *jvmti = NULL;
	if (JNI_OK != (*vm)->GetEnv(vm, (void **) (&jvmti), JVMTI_VERSION_1_2)) {
		ERROT_EXIT(JVMTI_ERR, "ERROR: Unable to access JVMTI.\n");
	}

	INFO_LOG("Parse config...");
	if (0 != parse_config(options)) {
		ERROT_EXIT(CONFIG_ERR, "ERROR: Parse config file failed.\n");
	}

	jvmtiCapabilities capabilities;
	memset(&capabilities, 0, sizeof(jvmtiCapabilities));
	capabilities.can_access_local_variables = 1;
	capabilities.can_generate_breakpoint_events = 1;
	capabilities.can_generate_exception_events = 1;
	capabilities.can_generate_field_access_events = 1;
	capabilities.can_get_bytecodes = 1;
	capabilities.can_get_constant_pool = 1;
	capabilities.can_get_line_numbers = 1;

	if (checkJVMTIError(jvmti, (*jvmti)->AddCapabilities(jvmti, &capabilities), UNABLE_SET_CAP)) {
		ERROT_EXIT(CAP_ERR, "Exit with %d", CAP_ERR);
	}

	checkJVMTIError(
			jvmti,
			(*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_EXCEPTION, NULL),
			"Cannot set Event Exception Notification"
	);
	checkJVMTIError(
			jvmti,
			(*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_INIT, NULL),
			"Cannot set VM init Notification"
	);
	checkJVMTIError(
			jvmti,
			(*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_BREAKPOINT, NULL),
			"Cannot set Event Breakpoint Notification"
	);
	checkJVMTIError(
			jvmti,
			(*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_PREPARE, NULL),
			"Cannot set Event Class Prepare Notification"
	);

	jvmtiEventCallbacks callbacks;
	memset(&callbacks, 0, sizeof(jvmtiEventCallbacks));
	callbacks.Breakpoint = &callbackEventBreakpoint;
	callbacks.ClassPrepare = &callbackClassPrepare;

	if (checkJVMTIError(jvmti, (*jvmti)->SetEventCallbacks(jvmti, &callbacks, sizeof(jvmtiEventCallbacks)), CANNOT_SET_CALLBACKS)) {
		ERROT_EXIT(CALL_ERR, "Exit with %d", CALL_ERR);
	}

	return JNI_OK;
}