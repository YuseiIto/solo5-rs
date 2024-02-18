DOCKER := docker

image: .image-built

.image-built: Dockerfile
	$(DOCKER) build -t solo5:latest .
	touch .image-built

.PHONY: clean
clean:
	rm -f .image-built

