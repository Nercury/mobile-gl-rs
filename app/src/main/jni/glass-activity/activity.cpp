#include "activity.h"

#include <android/log.h>

#define LOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, "threaded_app", __VA_ARGS__))
#define LOGE(...) ((void)__android_log_print(ANDROID_LOG_ERROR, "threaded_app", __VA_ARGS__))

/* For debug builds, always enable the debug traces in this library */
#ifndef NDEBUG
#  define LOGV(...)  ((void)__android_log_print(ANDROID_LOG_VERBOSE, "threaded_app", __VA_ARGS__))
#else
#  define LOGV(...)  ((void)0)
#endif

void ANativeActivity_onCreate(ANativeActivity* activity,
                              void* savedState, size_t savedStateSize) {
    LOGV("Hellow from activity: %p\n", activity);

//    activity->callbacks->onDestroy = onDestroy;
//    activity->callbacks->onStart = onStart;
//    activity->callbacks->onResume = onResume;
//    activity->callbacks->onSaveInstanceState = onSaveInstanceState;
//    activity->callbacks->onPause = onPause;
//    activity->callbacks->onStop = onStop;
//    activity->callbacks->onConfigurationChanged = onConfigurationChanged;
//    activity->callbacks->onLowMemory = onLowMemory;
//    activity->callbacks->onWindowFocusChanged = onWindowFocusChanged;
//    activity->callbacks->onNativeWindowCreated = onNativeWindowCreated;
//    activity->callbacks->onNativeWindowDestroyed = onNativeWindowDestroyed;
//    activity->callbacks->onInputQueueCreated = onInputQueueCreated;
//    activity->callbacks->onInputQueueDestroyed = onInputQueueDestroyed;

    //activity->instance = android_app_create(activity, savedState, savedStateSize);
}