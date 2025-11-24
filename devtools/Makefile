TAG := $(shell git log -1 --pretty=%h HEAD) 
UBUNTU_BASE_IMAGE := devmatteini/dra-ubuntu-base

UBUNTU_BASE_IMAGE_TAG := ${UBUNTU_BASE_IMAGE}:${TAG}
UBUNTU_BASE_IMAGE_LATEST := ${UBUNTU_BASE_IMAGE}:latest

debug:
	@echo "Tag" ${TAG}
	@echo "Ubuntu image" ${UBUNTU_BASE_IMAGE}
	@echo "Ubuntu image tagged" ${UBUNTU_BASE_IMAGE_TAG}
	@echo "Ubuntu image latest" ${UBUNTU_BASE_IMAGE_LATEST}

build-base:
	docker build -t ${UBUNTU_BASE_IMAGE} -f ./Dockerfile.ubuntu-base .

tag-base:
	docker tag ${UBUNTU_BASE_IMAGE} ${UBUNTU_BASE_IMAGE_TAG}
	docker tag ${UBUNTU_BASE_IMAGE} ${UBUNTU_BASE_IMAGE_LATEST}

push-base:
	docker push ${UBUNTU_BASE_IMAGE_TAG}
	docker push ${UBUNTU_BASE_IMAGE_LATEST}

.PHONY: debug build-base tag-base push-base
