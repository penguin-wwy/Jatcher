//
// Created by penguin on 19-12-25.
//

#ifndef JATCHER_TOOLS_H
#define JATCHER_TOOLS_H

#include <memory.h>

#include "bridge.h"

static char buffer[128] = {0};

#define RESET_STRING(MESSAGE...)	\
	memset(buffer, 0, 128);			\
	sprintf(buffer, MESSAGE)

#define ERROT_EXIT()
#endif //JATCHER_TOOLS_H
