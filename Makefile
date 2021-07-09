TEMP_FOLDER = temp
MOUNT_FOLDER = mnt
FS_IMG = fs.img

PROG_DIRS = $(filter-out $(TEMP_FOLDER)/ $(MOUNT_FOLDER)/, $(dir $(wildcard */.)))
PROG_NAME = $(foreach SRC, $(PROG_DIRS), $(patsubst %/, %, $(SRC)))
PROG_BINS = $(foreach NAME, $(PROG_NAME), $(TEMP_FOLDER)/$(NAME))

$(MOUNT_FOLDER):
	mkdir $(MOUNT_FOLDER)

$(TEMP_FOLDER):
	mkdir $(TEMP_FOLDER)

$(FS_IMG): $(PROG_BINS) $(MOUNT_FOLDER)
	sudo dd if=/dev/zero of=$(FS_IMG) bs=1024 count=1048576
	sudo mkfs.vfat $(FS_IMG)
	sudo mount -o loop $(FS_IMG) $(MOUNT_FOLDER)
	sudo rm -rf $(MOUNT_FOLDER)/*
	sudo cp -r $(TEMP_FOLDER)/* $(MOUNT_FOLDER)
	sudo umount $(MOUNT_FOLDER)
	sudo chmod 777 $(FS_IMG)

$(PROG_BINS): $(TEMP_FOLDER)/%: % $(TEMP_FOLDER)
	@echo "\033[92m==================== making $@ ====================\033[0m"
	make -C $< all
	cp $</$< $@

all: $(FS_IMG)

clean:
	rm -rf $(TEMP_FOLDER) $(MOUNT_FOLDER) $(FS_IMG)
	for DIR in $(PROG_DIRS); \
		do make -C $$DIR clean; \
	done

.PHONY:
	clean all