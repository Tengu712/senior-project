#pragma once

#define ERROR_IF(condExpr, funName, message, destruction, retValue) \
    if ((condExpr)) {                                               \
        printf("[ error ] %s: %s\n", (funName), (message));         \
        destruction;                                                \
        return (retValue);                                          \
    }
