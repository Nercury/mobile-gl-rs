#include "activity.h"

extern "C" {
void android_activity_create(ANativeActivity *activity,
                             void *savedState, size_t savedStateSize);
}

void ANativeActivity_onCreate(ANativeActivity *activity,
                              void *savedState, size_t savedStateSize) {
    android_activity_create(activity, savedState, savedStateSize);
}