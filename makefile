.DEFAULT_GOAL := help
PROJECTNAME=$(shell basename "$(PWD)")
MAIN_SOURCE=src/lib.rs
SOURCES=$(sort $(wildcard ./src/**/*.rs))
TARGETS=encryption | ffi | hashing | signing | wallet | snarks

OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')
PATH := $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin:$(PATH)

ANDROID_AARCH64_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/aarch64-linux-android29-clang
ANDROID_ARMV7_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/armv7a-linux-androideabi29-clang
ANDROID_I686_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/i686-linux-android29-clang
ANDROID_X86_64_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/x86_64-linux-android29-clang

SHELL := /bin/bash

# ##############################################################################
# # GENERAL
# ##############################################################################

.PHONY: help
help: makefile
	@echo
	@echo " Available actions in "$(PROJECTNAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo

## init: Install missing dependencies
.PHONY: init
init:
	@if [ $$(uname) == "Darwin" ] ; then rustup target add aarch64-apple-ios x86_64-apple-ios ; fi
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	@if [ $$(uname) == "Darwin" ] ; then cargo install cargo-lipo ; fi
	cargo install cbindgen
	make link target=ffi

## :

# ##############################################################################
# # RECIPES
# ##############################################################################

## all: Compile all the components for iOS, Android and their C bindings
all:
	make encryption
	make ffi
	make hashing
	make signing
	make snarks
	make wallet

## link: Link the project to a specific component (make link target=<component>)
link: 
	make src/lib.rs Cargo.toml target=$(target)

## :

## encryption: Compile iOS, Android and bindings for encryption
encryption:
	make link target=encryption
	make ios android target=encryption
	make bindings artifacts target=encryption
## ffi: Compile iOS, Android and bindings for ffi
ffi:
	make link target=ffi
	make ios android target=ffi
	make bindings artifacts target=ffi
## hashing: Compile iOS, Android and bindings for hashing
hashing:
	make link target=hashing
	make ios android target=hashing
	make bindings artifacts target=hashing
## signing: Compile iOS, Android and bindings for signing
signing:
	make link target=signing
	make ios android target=signing
	make bindings artifacts target=signing
## snarks: Compile iOS, Android and bindings for snarks
snarks:
	make link target=snarks
	make ios android target=snarks
	make bindings artifacts target=snarks
## wallet: Compile iOS, Android and bindings for wallet
wallet:
	make link target=wallet
	make ios android target=wallet
	make bindings artifacts target=wallet


.PHONY: src/lib.rs
src/lib.rs: 
	@case "$(target)" in \
		$(TARGETS)) ;; \
		*) echo "ERROR: Invalid target parameter: $(target)" >&2 ; false ;; \
	esac
	
	rm -f $@
	ln -s components/$(target).rs $@

.PHONY: Cargo.toml
Cargo.toml: 
	@case "$(target)" in \
		$(TARGETS)) ;; \
		*) echo "ERROR: Invalid target parameter: $(target)" >&2 ; false ;; \
	esac
	
	rm -f $@
	ln -s src/components/Cargo-$(target).toml $@

## :

## ios: Compile the iOS targets (aarch64 and x86_64)
ios: target/universal/release/libdvote$(target).a

target/universal/release/libdvote$(target).a: $(SOURCES) ndk-home
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo lipo --release ; \
	else echo "Skipping iOS compilation on $$(uname)" ; \
	fi

## android: Compile the android targets (aarch64, armv7, i686 and x86_64)
android: target/aarch64-linux-android/release/libdvote$(target).so target/armv7-linux-androideabi/release/libdvote$(target).so target/i686-linux-android/release/libdvote$(target).so target/x86_64-linux-android/release/libdvote$(target).so

target/aarch64-linux-android/release/libdvote$(target).so: $(SOURCES) ndk-home
	CC_aarch64_linux_android=$(ANDROID_AARCH64_LINKER) \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(ANDROID_AARCH64_LINKER) \
		cargo build --target aarch64-linux-android --release

target/armv7-linux-androideabi/release/libdvote$(target).so: $(SOURCES) ndk-home
	CC_armv7_linux_androideabi=$(ANDROID_ARMV7_LINKER) \
	CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(ANDROID_ARMV7_LINKER) \
		cargo build --target armv7-linux-androideabi --release

target/i686-linux-android/release/libdvote$(target).so: $(SOURCES) ndk-home
	CC_i686_linux_android=$(ANDROID_I686_LINKER) \
	CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$(ANDROID_I686_LINKER) \
		cargo  build --target i686-linux-android --release

target/x86_64-linux-android/release/libdvote$(target).so: $(SOURCES) ndk-home
	CC_x86_64_linux_android=$(ANDROID_X86_64_LINKER) \
	CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$(ANDROID_X86_64_LINKER) \
		cargo build --target x86_64-linux-android --release
		
.PHONY: ndk-home
ndk-home:
	@if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
		echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
		exit 1 ; \
	fi

## bindings: Generate the .h binding files for iOS
bindings: target/bindings-$(target).h

target/bindings-$(target).h: $(MAIN_SOURCE)
	cbindgen $^ -c cbindgen.toml | grep -v \#include | uniq > $@

.PHONY: artifacts
artifacts:
	@case "$(target)" in \
		$(TARGETS)) ;; \
		*) echo "ERROR: Invalid target parameter: $(target)" >&2 ; false ;; \
	esac

	mkdir -p $@
	ln -f -s ../target/aarch64-linux-android/release/libdvote$(target).so $@/libdvote$(target)-aarch64.so
	ln -f -s ../target/armv7-linux-androideabi/release/libdvote$(target).so $@/libdvote$(target)-armv7.so
	ln -f -s ../target/i686-linux-android/release/libdvote$(target).so $@/libdvote$(target)-i686.so
	ln -f -s ../target/x86_64-linux-android/release/libdvote$(target).so $@/libdvote$(target)-x86_64.so
	ln -f -s ../target/universal/release/libdvote$(target).a $@
	ln -f -s ../target/bindings-$(target).h $@

## :

# ##############################################################################
# # OTHER
# ##############################################################################

## clean: Clean the rust artifacts and the bindings
.PHONY: clean
clean:
	cargo clean
	rm -f target/binding*
