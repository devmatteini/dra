TAG := $(shell git log -1 --pretty=%h HEAD) 
BASE_IMAGE := devmatteini/dra-ubuntu-base

BASE_IMAGE_TAG := ${BASE_IMAGE}:${TAG}
BASE_IMAGE_LATEST := ${BASE_IMAGE}:latest

debug:
	@echo "Tag" ${TAG}
	@echo "Image" ${BASE_IMAGE}
	@echo "Image tagged" ${BASE_IMAGE_TAG}
	@echo "Image latest" ${BASE_IMAGE_LATEST}

build-base:
	docker build -t ${BASE_IMAGE} -f ./Dockerfile.ubuntu-base .

tag-base:
	docker tag ${BASE_IMAGE} ${BASE_IMAGE_TAG}
	docker tag ${BASE_IMAGE} ${BASE_IMAGE_LATEST}

push-base:
	docker push ${BASE_IMAGE_TAG}
	docker push ${BASE_IMAGE_LATEST}

.PHONY: debug build-base tag-base push-base
