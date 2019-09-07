# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

### Changed


## [v0.2.1] - 2019-09-06

### Changed
- Removed uses of `mem::uninitialized`.
- Fixed bug `math::trig::sin_u16` resulting in a weird results near its peaks (Thanks @vurpo !).
- Fix divide by 0 error in `ColorRGB::maximize_brightness`.
- Renaming of math functions from `<method name><integer size>` to `<method name>_u<integer size>`

## [v0.2.0] - 2018-04-26

Initial Release - CHANGELOG starts