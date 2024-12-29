# SPDX-License-Identifier: GPL-2.0-only

# Add necessary flags for trace events and other configurations
ccflags-y += -I$(src)			# needed for trace events

# Enable support for Binder filesystem (BinderFS)
obj-$(CONFIG_ANDROID_BINDERFS)		+= binderfs.o

# Enable support for Binder IPC mechanism
obj-$(CONFIG_ANDROID_BINDER_IPC)		+= binder.o binder_alloc.o

# Enable Binder IPC selftests (for debugging purposes)
obj-$(CONFIG_ANDROID_BINDER_IPC_SELFTEST) += binder_alloc_selftest.o

# Optionally, enable support for new features or debugging tools
# For example, enabling Binder logging or additional trace events if needed:
# obj-$(CONFIG_ANDROID_BINDER_LOGGING) += binder_log.o
# obj-$(CONFIG_ANDROID_BINDER_TRACE) += binder_trace.o

# Add any other necessary files or features for your platform
# obj-$(CONFIG_ANDROID_FEATURE_X) += feature_x.o

# Clean rule to remove intermediate files, if needed
clean-files := *.o
