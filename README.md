# marytts-pico
A simple wrapper around PicoTTS that implements the MaryTTS interface

This is to fill a need for a local and lightweight TTS service for Home Assistant, that works with Home Assistant Container.

MaryTTS is a rather heavy image, both size and memory constraints, but PicoTTS needs to be installed locally, which isn't compatible with Home Assistant Container, as least if you don't want to build your own image every time there is a new build.

This is a braindead implementation of the bits of the MaryTTS API, that wraps the pico2wave executable. The Docker image is based on alpine and has just what is needed, the image size is only about 25MB - fast and light.
