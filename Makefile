CURRENT_DIR=$(shell pwd)
FROM_DIRECTORY=$(shell ls projects/)
TO_DIRECTORY=${CURRENT_DIR}/code

all:
	for dir in ${FROM_DIRECTORY} ; do \
		mkdir -p ${TO_DIRECTORY}/$$dir ; \
		cp ${CURRENT_DIR}/projects/$$dir/src/*.rs ${TO_DIRECTORY}/$$dir ; \
    done
