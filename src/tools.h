//
// Created by penguin on 19-12-25.
//

#ifndef JATCHER_TOOLS_H
#define JATCHER_TOOLS_H

#include <stdlib.h>
#include <memory.h>

#include <jni.h>
#include <jvmti.h>

#include "bridge.h"

#define BUFFER_SIZE	256

static char buffer[BUFFER_SIZE] = {0};

enum {
	JVM_ERR	= 1,
	JVMTI_ERR,
	FILE_ERR,
	CAP_ERR,
	CALL_ERR,
	CONFIG_ERR,
};

#define RESET_STRING(MESSAGE...)	\
	memset(buffer, 0, BUFFER_SIZE);			\
	sprintf(buffer, MESSAGE)

#define ERROT_EXIT(ERR_CODE, MESSAGE...)	\
	do {									\
		RESET_STRING(MESSAGE);				\
		error_log(buffer);					\
		exit(ERR_CODE);						\
	} while (0)

#define ERROR_LOG(MESSAGE)		\
	do {						\
		RESET_STRING(MESSAGE);	\
		error_log(buffer);		\
	} while (0)

#define INFO_LOG(MESSAGE...)	\
	do {						\
		RESET_STRING(MESSAGE);	\
		info_log(buffer);		\
	} while (0)

#define WARN_LOG(MESSAGE...)	\
	do {						\
		RESET_STRING(MESSAGE);	\
		warn_log(buffer);		\
	} while (0)

int checkJVMTIError(jvmtiEnv *jvmti, jvmtiError err_code, const char *str);
#endif //JATCHER_TOOLS_H
