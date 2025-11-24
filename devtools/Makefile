TAG := $(shell git log -1 --pretty=%h HEAD) 
UBUNTU_BASE_IMAGE := devmatteini/dra-ubuntu-base
FEDORA_BASE_IMAGE := devmatteini/dra-fedora-base

UBUNTU_BASE_IMAGE_TAG := ${UBUNTU_BASE_IMAGE}:${TAG}
UBUNTU_BASE_IMAGE_LATEST := ${UBUNTU_BASE_IMAGE}:latest

FEDORA_BASE_IMAGE_TAG := ${FEDORA_BASE_IMAGE}:${TAG}
FEDORA_BASE_IMAGE_LATEST := ${FEDORA_BASE_IMAGE}:latest

debug:
	@echo "Tag" ${TAG}
	@echo "Ubuntu image" ${UBUNTU_BASE_IMAGE}
	@echo "Ubuntu image tagged" ${UBUNTU_BASE_IMAGE_TAG}
	@echo "Ubuntu image latest" ${UBUNTU_BASE_IMAGE_LATEST}
	@echo "Fedora image" ${FEDORA_BASE_IMAGE}
	@echo "Fedora image tagged" ${FEDORA_BASE_IMAGE_TAG}
	@echo "Fedora image latest" ${FEDORA_BASE_IMAGE_LATEST}

build-base:
	docker build -t ${UBUNTU_BASE_IMAGE} -f ./Dockerfile.ubuntu-base .
	docker build -t ${FEDORA_BASE_IMAGE} -f ./Dockerfile.fedora-base .

tag-base:
	docker tag ${UBUNTU_BASE_IMAGE} ${UBUNTU_BASE_IMAGE_TAG}
	docker tag ${UBUNTU_BASE_IMAGE} ${UBUNTU_BASE_IMAGE_LATEST}

	docker tag ${FEDORA_BASE_IMAGE} ${FEDORA_BASE_IMAGE_TAG}
	docker tag ${FEDORA_BASE_IMAGE} ${FEDORA_BASE_IMAGE_LATEST}

push-base:
	docker push ${UBUNTU_BASE_IMAGE_TAG}
	docker push ${UBUNTU_BASE_IMAGE_LATEST}

	docker push ${FEDORA_BASE_IMAGE_TAG}
	docker push ${FEDORA_BASE_IMAGE_LATEST}

.PHONY: debug build-base tag-base push-base
