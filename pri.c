#include <stdio.h>
#include <sys/types.h>
#include <sys/sysctl.h>
#include <mach/mach_init.h>
#include <mach/thread_act.h>

// Obtained from the Stage psychophysics suite for Matlab.

void setMaxPriority()
{
    int mib[2];
    int busFreq;
    size_t len;
    thread_time_constraint_policy_data_t policy;
    kern_return_t result;

    mib[0] = CTL_HW;
    mib[1] = HW_BUS_FREQ;
    len = sizeof(busFreq);
    sysctl(mib, 2, &busFreq, &len, NULL, 0);

    policy.period = busFreq / 120;
    policy.computation = policy.period * 0.9;
    policy.constraint = policy.computation;
    policy.preemptible = 1;

    result = thread_policy_set(mach_thread_self(), THREAD_TIME_CONSTRAINT_POLICY, (thread_policy_t)&policy, THREAD_TIME_CONSTRAINT_POLICY_COUNT);
    if (result)
    {
        printf("Failed to set max priority");
    }
}
