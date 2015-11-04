#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

#include <libcgroup.h>
#include <sys/types.h>
#include <unistd.h>

int box_current_process(const char* group, const char** resources) {
    pid_t pid = getpid();

    int ret = cgroup_init();
	if (ret) {
        fprintf(stderr, "libcgroup initialization failed: %s\n",
                cgroup_strerror(ret));
        return ret;
	}

    ret = cgroup_change_cgroup_path(group, pid, resources);
    if (ret) {
        fprintf(stderr, "cgroup change of group failed, errno=%d\n", ret);
        return ret;
    }

    char* path; 
    ret = cgroup_get_current_controller_path(pid, "cpu", &path);
    if (!ret) {
        printf("%s\n", path);
        free(path);
    }

    return 0;
}

int main()
{
    const char* resources[] = {"cpu", "memory", NULL};
    box_current_process("biye", resources);
    return 0;
}
