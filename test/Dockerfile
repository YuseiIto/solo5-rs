FROM alpine:3.19
RUN apk update
RUN apk upgrade
RUN apk add git
RUN apk add build-base pkgconfig libseccomp-dev linux-headers
RUN mkdir -p /workspace
WORKDIR /workspace
RUN git clone https://github.com/Solo5/solo5.git
WORKDIR /workspace/solo5
RUN ./configure.sh
RUN make
RUN make install
