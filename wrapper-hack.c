#include <unistd.h>
#include <errno.h>
#include <stdio.h>
#include <dlfcn.h>

int execve(const char *path, char *const *argv, char *const *envp) {
  static int (*_execve)(const char *, char *const *, char *const *) = NULL;
  _execve = dlsym(RTLD_NEXT, "execve");

  printf("Called %s\n", path);

  return _execve(path, argv, envp);
}
