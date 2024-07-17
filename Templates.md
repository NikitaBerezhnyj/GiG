# .gitignore templates

- [.gitignore templates](#gitignore-templates)
  - [Programming languages](#programming-languages)
    - [C](#c)
    - [C++](#c-1)
    - [C#](#c-2)
    - [Java](#java)
    - [Kotlin](#kotlin)
    - [Swift](#swift)
    - [Rust](#rust)
    - [Python](#python)
    - [Go](#go)
    - [Ruby](#ruby)
    - [PHP](#php)
  - [Frameworks and libraries](#frameworks-and-libraries)
    - [React](#react)
    - [Angular](#angular)
    - [Vue.js](#vuejs)
    - [Svelte](#svelte)
    - [Node.js](#nodejs)
    - [Next.js](#nextjs)
    - [Nuxt.js](#nuxtjs)
    - [Express.js](#expressjs)
    - [Django](#django)
    - [Tauri](#tauri)
    - [Electron](#electron)
    - [Spring](#spring)
    - [Ruby on Rails](#ruby-on-rails)
    - [Laravel](#laravel)
    - [Flutter](#flutter)
    - [Flask](#flask)
    - [ASP.NET](#aspnet)
  - [Game engines](#game-engines)
    - [Unity](#unity)
    - [Unreal Engine](#unreal-engine)
    - [Godot](#godot)
    - [Ren'Py](#renpy)
  - [Development tools](#development-tools)
    - [Docker](#docker)
    - [npm](#npm)
    - [pnpm](#pnpm)
    - [Yarn](#yarn)
    - [Gradle](#gradle)
    - [Maven](#maven)
    - [Webpack](#webpack)
    - [Composer](#composer)
    - [pip](#pip)
  - [Database management systems](#database-management-systems)
    - [MySQL](#mysql)
    - [MongoDB](#mongodb)
    - [SQLite](#sqlite)
    - [Firebase](#firebase)
  - [IDE and code editors](#ide-and-code-editors)
    - [VS Code](#vs-code)
    - [JetBrains IDE's](#jetbrains-ides)
    - [Visual Studio](#visual-studio)
    - [Eclipse](#eclipse)
    - [Android Studio](#android-studio)
    - [Xcode](#xcode)
    - [Sublime Text](#sublime-text)
    - [Atom](#atom)
    - [Vim](#vim)
  - [Operating systems](#operating-systems)
    - [Windows](#windows)
    - [Linux](#linux)
    - [MacOS](#macos)
  - [Other](#other)
    - [Sass/SCSS](#sassscss)
    - [Less](#less)
    - [Jupyter Notebooks](#jupyter-notebooks)
    - [Terraform](#terraform)

## Programming languages

### C

```
*.o
*.obj
*.so
*.exe
```

### C++

```
*.d

*.slo
*.lo
*.o
*.obj

*.gch
*.pch

*.so
*.dylib
*.dll

*.mod
*.smod

*.lai
*.la
*.a
*.lib

*.exe
*.out
*.app
```

### C#

```
*.suo
*.user
*.sln.docstates

[Dd]ebug/
[Rr]elease/
x64/
[Bb]in/
[Oo]bj/

[Tt]est[Rr]esult*/
[Bb]uild[Ll]og.*

*_i.c
*_p.c
*_i.h
*.ilk
*.meta
*.obj
*.pch
*.pdb
*.pgc
*.pgd
*.rsp
*.sbr
*.tlb
*.tli
*.tlh
*.tmp
*.tmp_proj
*.log
*.vspscc
*.vssscc
.builds
*.pidb
*.log
*.svclog
*.scc

ipch/
*.aps
*.ncb
*.opensdf
*.sdf
*.cachefile

*.psess
*.vsp
*.vspx

# Guidance Automation Toolkit
*.gpState

# ReSharper is a .NET coding add-in
_ReSharper*/
*.[Rr]e[Ss]harper
*.DotSettings.user

# Click-Once directory
publish/

# Publish Web Output
*.Publish.xml
*.pubxml
*.azurePubxml

# NuGet Packages Directory
## TODO: If you have NuGet Package Restore enabled, uncomment the next line
packages/
## TODO: If the tool you use requires repositories.config, also uncomment the next line
!packages/repositories.config

# Windows Azure Build Output
csx/
*.build.csdef

# Windows Store app package directory
AppPackages/

# Others
sql/
*.Cache
ClientBin/
[Ss]tyle[Cc]op.*
![Ss]tyle[Cc]op.targets
~$*
*~
*.dbmdl
*.[Pp]ublish.xml

*.publishsettings

# RIA/Silverlight projects
Generated_Code/

# Backup & report files from converting an old project file to a newer
# Visual Studio version. Backup files are not needed, because we have git ;-)
_UpgradeReport_Files/
Backup*/
UpgradeLog*.XML
UpgradeLog*.htm

# SQL Server files
App_Data/*.mdf
App_Data/*.ldf

# =========================
# Windows detritus
# =========================

# Windows image file caches
Thumbs.db
ehthumbs.db

# Folder config file
Desktop.ini

# Recycle Bin used on file shares
$RECYCLE.BIN/

# Mac desktop service store files
.DS_Store

_NCrunch*
```

### Java

```
*.class

*.log

*.ctxt

.mtj.tmp/

*.jar
*.war
*.nar
*.ear
*.zip
*.tar.gz
*.rar
```

### Kotlin

```
*.class

*.log

*.ctxt

.mtj.tmp/

*.jar
*.war
*.nar
*.ear
*.zip
*.tar.gz
*.rar

hs_err_pid*
replay_pid*
```

### Swift

```
*.hmap

*.ipa
*.dSYM.zip
*.dSYM

timeline.xctimeline
playground.xcworkspace

.build/

Carthage/Build/

fastlane/report.xml
fastlane/Preview.html
fastlane/screenshots/**/*.png
fastlane/test_output
```

### Rust

```
debug/
target/

Cargo.lock

**/*.rs.bk

*.pdb
```

### Python

```
# Byte-compiled / optimized / DLL files
__pycache__/
*.py[cod]
*$py.class

# C extensions
*.so

# Distribution / packaging
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg
MANIFEST

# PyInstaller
#  Usually these files are written by a python script from a template
#  before PyInstaller builds the exe, so as to inject date/other infos into it.
*.manifest
*.spec

# Installer logs
pip-log.txt
pip-delete-this-directory.txt

# Unit test / coverage reports
htmlcov/
.tox/
.nox/
.coverage
.coverage.*
.cache
nosetests.xml
coverage.xml
*.cover
*.py,cover
.hypothesis/
.pytest_cache/
cover/

# Translations
*.mo
*.pot

# Django stuff:
*.log
local_settings.py
db.sqlite3
db.sqlite3-journal

# Flask stuff:
instance/
.webassets-cache

# Scrapy stuff:
.scrapy

# Sphinx documentation
docs/_build/

# PyBuilder
.pybuilder/
target/

# Jupyter Notebook
.ipynb_checkpoints

# IPython
profile_default/
ipython_config.py

# pyenv
#   For a library or package, you might want to ignore these files since the code is
#   intended to run in multiple environments; otherwise, check them in:
# .python-version

# pipenv
#   According to pypa/pipenv#598, it is recommended to include Pipfile.lock in version control.
#   However, in case of collaboration, if having platform-specific dependencies or dependencies
#   having no cross-platform support, pipenv may install dependencies that don't work, or not
#   install all needed dependencies.
#Pipfile.lock

# poetry
#   Similar to Pipfile.lock, it is generally recommended to include poetry.lock in version control.
#   This is especially recommended for binary packages to ensure reproducibility, and is more
#   commonly ignored for libraries.
#   https://python-poetry.org/docs/basic-usage/#commit-your-poetrylock-file-to-version-control
#poetry.lock

# pdm
#   Similar to Pipfile.lock, it is generally recommended to include pdm.lock in version control.
#pdm.lock
#   pdm stores project-wide configurations in .pdm.toml, but it is recommended to not include it
#   in version control.
#   https://pdm.fming.dev/latest/usage/project/#working-with-version-control
.pdm.toml
.pdm-python
.pdm-build/

# PEP 582; used by e.g. github.com/David-OConnor/pyflow and github.com/pdm-project/pdm
__pypackages__/

# Celery stuff
celerybeat-schedule
celerybeat.pid

# SageMath parsed files
*.sage.py

# Environments
.env
.venv
env/
venv/
ENV/
env.bak/
venv.bak/

# Spyder project settings
.spyderproject
.spyproject

# Rope project settings
.ropeproject

# mkdocs documentation
/site

# mypy
.mypy_cache/
.dmypy.json
dmypy.json

# Pyre type checker
.pyre/

# pytype static type analyzer
.pytype/

# Cython debug symbols
cython_debug/

# PyCharm
#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can
#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore
#  and can be added to the global gitignore or merged into this file.  For a more nuclear
#  option (not recommended) you can uncomment the following to ignore the entire idea folder.
#.idea/
```

### Go

```
*.exe
*.exe~
*.dll
*.so
*.dylib

*.test

*.out

go.work
```

### Ruby

```
*.gem
*.rbc
/.config
/coverage/
/InstalledFiles
/pkg/
/spec/reports/
/spec/examples.txt
/test/tmp/
/test/version_tmp/
/tmp/

.byebug_history

.dat*
.repl_history
build/
*.bridgesupport
build-iPhoneOS/
build-iPhoneSimulator/

/.yardoc/
/_yardoc/
/doc/
/rdoc/

/.bundle/
/vendor/bundle
/lib/bundler/man/
```

### PHP

```
# General
*.log
*.cache
*.session
*.bak
*.tmp

# PHPStorm
.idea/

# Composer
/vendor/
/composer.lock

# Node modules
/node_modules/

# Environment variables
.env

# Cache and logs
/tmp/cache/*
/tmp/logs/*
```

___

## Frameworks and libraries

### React

```
.DS_*
*.log
logs
**/*.backup.*
**/*.back.*

node_modules
bower_components

*.sublime*

psd
thumb
sketch
```

### Angular

```
dist/
tmp/
app/**/*.js
app/**/*.js.map

node_modules/
bower_components/

.idea/

.sass-cache/
connect.lock/
coverage/
libpeerconnection.log/
npm-debug.log
testem.log
typings/
.angular/

e2e/*.js
e2e/*.map

.DS_Store/
```

### Vue.js

```
node_modules/
dist/
npm-debug.log
yarn-error.log
```

### Svelte

```
# Node modules
node_modules/

# SvelteKit build output
/build/
/.svelte-kit/

# Sapper build outputs
__sapper__/

# SvelteKit/rollup/webpack cache
/.rollup.cache/
/.cache/

# Production
/build/
/public/build/

# Dependency directories
/.yarn/
.pnp.*

# IDE specific
.idea/
.vscode/

# Mac
.DS_Store

# Windows
Thumbs.db

# Log files
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# dotenv environment variables file
.env
.env.*
```

### Node.js

```
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
lerna-debug.log*
.pnpm-debug.log*

report.[0-9]*.[0-9]*.[0-9]*.[0-9]*.json

pids
*.pid
*.seed
*.pid.lock

lib-cov

coverage
*.lcov

.nyc_output

.grunt

bower_components

.lock-wscript

build/Release

node_modules/
jspm_packages/

web_modules/

*.tsbuildinfo

.npm

.eslintcache

.stylelintcache

.rpt2_cache/
.rts2_cache_cjs/
.rts2_cache_es/
.rts2_cache_umd/

.node_repl_history

*.tgz

.yarn-integrity

.env
.env.development.local
.env.test.local
.env.production.local
.env.local

.cache
.parcel-cache

.next
out

.nuxt
dist

.cache/

.vuepress/dist

.temp
.cache

.docusaurus

.serverless/

.fusebox/

.dynamodb/

.tern-port

.vscode-test

.yarn/cache
.yarn/unplugged
.yarn/build-state.yml
.yarn/install-state.gz
.pnp.*
```

### Next.js

```
# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Node modules
node_modules/

# Build output
.next/
out/

# Next.js build output
.next/
out/

# Dependency directories
node_modules/
.pnp
.pnp.js

# Production
/build/
public/build/

# Cache directories
.cache/
.parcel-cache/

# Coverage directory used by tools like istanbul
coverage/
*.lcov

# Optional npm cache directory
.npm
.npmrc

# Optional eslint cache
.eslintcache

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variables file
.env
.env.*
!.env.example

# Vercel deployment
.vercel/

# IDE specific
.idea/
.vscode/

# System files
.DS_Store
Thumbs.db

# Miscellaneous
.swp
*~
```

### Nuxt.js

```
# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Dependency directories
node_modules/

# Nuxt build output
.nuxt/
dist/

# Cache directories
.cache/
.parcel-cache/

# Coverage directory used by tools like istanbul
coverage/
*.lcov

# Optional npm cache directory
.npm
.npmrc

# Optional eslint cache
.eslintcache

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variables file
.env
.env.*

# IDE specific
.idea/
.vscode/

# System files
.DS_Store
Thumbs.db

# Miscellaneous
.swp
*~

# Vercel deployment
.vercel/

# Nuxt generate output
dist/
```

### Express.js

```
# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Directory for instrumented libs generated by jscoverage/JSCover
lib-cov

# Coverage directory used by tools like istanbul
coverage
*.lcov

# nyc test coverage
.nyc_output

# Grunt intermediate storage (https://gruntjs.com/creating-plugins#storing-task-files)
.grunt

# Bower dependency directory (https://bower.io/)
bower_components

# node-waf configuration
.lock-wscript

# Compiled binary addons (https://nodejs.org/api/addons.html)
build/Release

# Dependency directories
node_modules/
jspm_packages/

# Snowpack dependency directory (https://snowpack.dev/)
web_modules/

# TypeScript cache
*.tsbuildinfo

# Optional npm cache directory
.npm

# Optional eslint cache
.eslintcache

# Optional REPL history
.node_repl_history

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variables file
.env
.env.test
.env.production

# parcel-bundler cache (https://parceljs.org/)
.cache
.parcel-cache

# Next.js build output
.next
out

# Nuxt.js build / generate output
.nuxt
dist

# Gatsby files
.cache/
public

# vuepress build output
.vuepress/dist

# Serverless directories
.serverless/

# FuseBox cache
.fusebox/

# DynamoDB Local files
.dynamodb/

# TernJS port file
.tern-port

# VS Code
.vscode/

# WebStorm
.idea/

# macOS
.DS_Store

# Windows
Thumbs.db
```

### Django

```
*.log
*.pot
*.pyc
__pycache__/
local_settings.py
db.sqlite3
db.sqlite3-journal
media

*.py[cod]
*$py.class

*.so

.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg
MANIFEST

*.manifest
*.spec

pip-log.txt
pip-delete-this-directory.txt

htmlcov/
.tox/
.nox/
.coverage
.coverage.*
.cache
nosetests.xml
coverage.xml
*.cover
*.py,cover
.hypothesis/
.pytest_cache/
cover/

*.mo

instance/
.webassets-cache

.scrapy


docs/_build/

.pybuilder/
target/

.ipynb_checkpoints

profile_default/
ipython_config.py

.pdm.toml

__pypackages__/

celerybeat-schedule
celerybeat.pid

*.sage.py

.env
.venv
env/
venv/
ENV/
env.bak/
venv.bak/

.spyderproject
.spyproject

.ropeproject

/site

.mypy_cache/
.dmypy.json
dmypy.json

.pyre/

.pytype/

cython_debug/
```

### Tauri

```
# Node
node_modules/
dist/
*.log

# Byte-compiled / optimized / DLL files
*.py[cod]
*.swp
*~

# Rust
/target/
*.rs.bk

# Tauri
/src-tauri/target/
**/node_modules/

# Build output
/src-tauri/target/release/
**/src-tauri/target/release/
**/src-tauri/target/debug/
**/src-tauri/target/debug/
**/src-tauri/target/**/build/
**/src-tauri/target/**/.fingerprint/
**/src-tauri/target/**/.cargo-lock
/src-tauri/.cargo-lock
/src-tauri/Cargo.lock

# Other
*.d.ts
*.tsbuildinfo
.yarn/
.pnp.*
.pnp

# System files
.DS_Store
Thumbs.db
```

### Electron

```
# Logs
*.log

# Electron files
node_modules/
/dist/

# OS generated files
.DS_Store
Thumbs.db

# User-specific files
.idea/
.vscode/

# Dependency directories
/node_modules
/jspm_packages

# Optional eslint cache
.eslintcache

# dotenv environment variables file
.env
.env.local
.env.*.local

# Debug files
npm-debug.log*
yarn-debug.log*
yarn-error.log*
```

### Spring

```
# Compiled class files
*.class

# Log files
*.log

# BlueJ files
*.ctxt

# Mobile Tools for Java (J2ME)
.mtj.tmp/

# Package Files #
*.jar
*.war
*.nar
*.ear
*.zip
*.tar.gz
*.rar

# virtual machine crash logs, see http://www.java.com/en/download/help/error_hotspot.xml
hs_err_pid*

# Maven
target/
pom.xml.tag
pom.xml.releaseBackup
pom.xml.versionsBackup
pom.xml.next
release.properties
dependency-reduced-pom.xml
buildNumber.properties
.mvn/timing.properties
.mvn/wrapper/maven-wrapper.jar
.mvn/wrapper/maven-wrapper.properties
.mvn/wrapper/MavenWrapperDownloader.java

# Gradle
.gradle
build/
!**/src/main/**/build/
!**/src/test/**/build/
!**/src/intTest/**/build/
!**/src/integrationTest/**/build/
!**/src/functionalTest/**/build/
!**/src/e2eTest/**/build/
!**/src/acceptanceTest/**/build/
!**/src/uiTest/**/build/
.gradle/
build/
!**/src/**/build/
!**/test-results/
!**/out/
.gradle/
out/
build/
!**/src/**/build/

# Gradle Wrapper
.gradle/
gradle/wrapper/gradle-wrapper.jar
!gradle/wrapper/gradle-wrapper.properties
!gradle/wrapper/MavenWrapperDownloader.java

# Intellij
.idea/
*.iws
*.iml
*.ipr

# NetBeans
nbproject/private/
build/
nbbuild/
dist/
nbdist/
.nb-gradle/
**/nbproject/private/
**/build/
**/nbbuild/
**/dist/
**/nbdist/
**/.nb-gradle/

# VS Code
.vscode/

# Mac
.DS_Store

# Windows
Thumbs.db
ehthumbs.db

# Eclipse
.project
.classpath
.settings/
bin/
tmp/

# STS (Spring Tool Suite)
.springBeans
.apt_generated/
.sts4-cache/
.target/
.apt_generated/
```

### Ruby on Rails

```
# Ignore bundler config
/.bundle

# Ignore the default SQLite database.
/db/*.sqlite3
/db/*.sqlite3-journal

# Ignore all logfiles and tempfiles.
/log/*
/tmp/*
!/log/.keep
!/tmp/.keep

# Ignore uploaded files in development
/storage/*
!/storage/.keep

# Ignore pidfiles, but keep the directory.
/tmp/pids/*
!/tmp/pids/
!/tmp/pids/.keep

# Ignore uploaded files stored on the local file system
/public/system
/public/uploads

# Ignore master key which is used for decrypting credentials and more.
/config/master.key

# Ignore node_modules
/node_modules
/yarn-error.log

# Ignore precompiled assets
/public/assets

# Ignore dotenv files
.env
.env.local
.env.*.local

# Ignore byebug command history file.
.byebug_history

# Ignore bundler and spring files
/.bundle/
/.sass-cache/
/.ruby-gemset
.ruby-version
.rspec
/spec/examples.txt
/vendor/bundle
/vendor/cache
/vendor/gems
/vendor/ruby

# Ignore RubyMine project files
.idea/

# Ignore Mac and Windows system files
.DS_Store
Thumbs.db

# Ignore other system files
*.swp
*~
```

### Laravel

```
# Laravel specific
/vendor
/node_modules
/public/storage
/public/hot
/public/build
/storage/*.key
/storage/app/public
/storage/debugbar
/storage/framework/cache/data
/storage/framework/sessions
/storage/framework/testing
/storage/framework/views
/storage/logs
/storage/clockwork

# Laravel 5 & 6 specific
/.env
.phpunit.result.cache
/.phpunit.result.cache

# Laravel 7 & 8 specific
/.env.example
/.env.testing
/.env.dusk.local
/.env.dusk.testing
/.phpunit.result.cache
.phpunit.result.cache
/phpunit.result.cache

# Environment variables
.env
.env.*
.env.local
.env.staging
.env.production
.env.testing
.env.dusk.local
.env.dusk.testing

# Laravel Mix generated files
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Node
node_modules/

# Compiled JS and CSS
/public/js/*.js
/public/css/*.css

# Cache and logs
/storage/*.key
/storage/*.log

# Laravel Telescope
/telescope

# IDE specific
.idea
.vscode

# System files
.DS_Store
Thumbs.db

# Other files and directories
Homestead.yaml
Homestead.json
```

### Flutter

```
# Flutter/Dart/Pub related
**/doc/api/

# Generated by pub
.dart_tool/
.packages
.pub-cache/
.pub/
build/

# Ongoing and result of dartdoc
dartdoc/

# Symbolication files
*.symbols

# Obfuscation symbol map
*.map.json

# Android related
**/android/**/gradle-wrapper.jar
**/android/.gradle
**/android/captures/
**/android/gradlew
**/android/gradlew.bat
**/android/local.properties
**/android/*.iml
**/android/.idea/
**/android/app/google-services.json
**/android/app/src/debug/*.keystore
**/android/app/src/profile/*.keystore
**/android/app/src/release/*.keystore
**/android/key.properties
**/android/app/src/debug/res/raw/keys.xml
**/android/app/src/profile/res/raw/keys.xml
**/android/app/src/release/res/raw/keys.xml
**/android/key.properties

# iOS related
**/ios/.generated/
**/ios/.last_build_id
**/ios/Flutter/.symlinks
**/ios/Flutter/ephemeral/
**/ios/Flutter/Flutter.framework/
**/ios/Flutter/App.framework/
**/ios/Flutter/Flutter.podspec
**/ios/Flutter/Flutter-Generated.xcconfig
**/ios/.dart_tool/
**/ios/Flutter/Flutter-Generated.xcconfig
**/ios/Flutter/flutter_export_environment.sh
**/ios/Flutter/Debug.xcconfig
**/ios/Flutter/Release.xcconfig
**/ios/Pods/
**/ios/Podfile.lock
**/ios/Podfile

# Xcode related
**/ios/Flutter/App.framework
**/ios/Flutter/Flutter.framework
**/ios/.symlinks/
**/ios/Flutter/Generated.xcconfig
**/ios/Flutter/ephemeral/
**/ios/.dart_tool/
**/ios/Flutter/flutter_export_environment.sh
**/ios/Flutter/Flutter.podspec
**/ios/Flutter/Flutter.xcworkspace
**/ios/Flutter/Flutter.podspec

# VS Code related
.vscode/

# IntelliJ related
.idea/
*.iml

# macOS related
.DS_Store

# Windows related
Thumbs.db

# Linux related
.snap/

# Temporary files
*.tmp
*.temp
```

### Flask

```
# Byte-compiled / optimized / DLL files
__pycache__/
*.py[cod]
*$py.class

# Environment variables
.env

# IDE specific files
.vscode/
.idea/

# Logs
*.log

# Python packaging and distribution
env/
build/
dist/
*.egg-info/

# Compiled Python files
*.pyc

# Unit test / coverage reports
.coverage
htmlcov/

# Flask specific
instance/
.pytest_cache/

# System and backup files
.DS_Store
Thumbs.db
```

### ASP.NET

```
# ASP.NET created files
*.user
*.obj
*.dll
*.exe

# Build results
[Dd]ebug/
[Dd]ebugPublic/
[Rr]elease/
[Rr]eleases/
x64/
x86/
[Aa][Rr][Mm]/
[Aa][Rr][Mm]64/
bld/
[Bb]in/
[Oo]bj/
[Ll]og/
[Ll]ogs/

# Visual Studio files
*.vscode/
.vs/
*.suo
*.ntvs*
*.njsproj
*.sln.docstates
*.suo
*.user
*.userosscache
*.sln.docstates
*.userprefs
*.ide

# Rider
.idea/

# ReSharper
_ReSharper*/
*.[Rr]e[Ss]harper
*.DotSettings

# Test generated files
[Tt]est[Rr]esult*/
[Bb]uild[Ll]og.*
[Mm]edium[Tt]rust.config

# ASP.NET Identity
/App_Data/*.mdf
/App_Data/*.ldf

# NuGet
packages/
*.nupkg
!.nuget/

# Visual Studio Code
.vscode/

# Rider
.idea/

# Rider
.idea/

# User-specific files
*.suo
*.user
*.userosscache
*.sln.docstates
*.userprefs
.idea/
.vscode/
```

___

## Game engines

### Unity

```
/[Ll]ibrary/
/[Tt]emp/
/[Oo]bj/
/[Bb]uild/
/[Bb]uilds/
/[Ll]ogs/
/[Uu]ser[Ss]ettings/

/[Mm]emoryCaptures/

/[Rr]ecordings/

/[Aa]ssets/Plugins/Editor/JetBrains*

.vs/

.gradle/

ExportedObj/
.consulo/
*.csproj
*.unityproj
*.sln
*.suo
*.tmp
*.user
*.userprefs
*.pidb
*.booproj
*.svd
*.pdb
*.mdb
*.opendb
*.VC.db

*.pidb.meta
*.pdb.meta
*.mdb.meta

sysinfo.txt

*.apk
*.aab
*.unitypackage
*.app

crashlytics-build.properties

/[Aa]ssets/[Aa]ddressable[Aa]ssets[Dd]ata/*/*.bin*

/[Aa]ssets/[Ss]treamingAssets/aa.meta
/[Aa]ssets/[Ss]treamingAssets/aa/*
```

### Unreal Engine

```
.vs/

*.slo
*.lo
*.o
*.obj

*.gch
*.pch

*.so
*.dylib
*.dll

*.mod

*.lai
*.la
*.a
*.lib

*.exe
*.out
*.app
*.ipa

*.xcodeproj
*.xcworkspace
*.sln
*.suo
*.opensdf
*.sdf
*.VC.db
*.VC.opendb

SourceArt/**/*.png
SourceArt/**/*.tga

Binaries/*
Plugins/**/Binaries/*

Build/*

!Build/*/
Build/*/**
!Build/*/PakBlacklist*.txt

!Build/**/*.ico

*_BuiltData.uasset

Saved/*

Intermediate/*
Plugins/**/Intermediate/*

DerivedDataCache/*
```

### Godot

```
.godot/

.import/
export.cfg
export_presets.cfg

*.translation

.mono/
data_*/
mono_crash.*.json
```

### Ren'Py

```
cache/
saves/

*.com
*.class
*.dll
*.exe
*.o
*.so

*.rpyc
*.rpymc

log.txt
errors.txt
traceback.txt
files.txt
```

___

## Development tools

### Docker

```
# Ignore local .env files
.env

# Ignore Docker Compose override files
docker-compose.override.yml
docker-compose.override.yaml
docker-compose.override.*

# Ignore log files
*.log

# Ignore files generated by Docker
docker-compose.yml
docker-compose.yaml
docker-compose.*.yml
docker-compose.*.yaml
.dockerignore

# Ignore .env file used in containers
.env.docker

# Ignore local .env file for development
.env.local

# Ignore .vscode folder
.vscode/

# Ignore .idea folder
.idea/

# Ignore .git folder (not necessary, just in case)
.git/

# Ignore .DS_Store (Mac specific)
.DS_Store

# Ignore Thumbs.db (Windows specific)
Thumbs.db
```

### npm

```
# dependencies
/node_modules

# testing
/coverage

# production
/build

# misc
npm-debug.log*
yarn-error.log*
.env
```

### pnpm

```
# pnpm store
.pnpm-store

# dependencies
/node_modules

# testing
/coverage

# production
/build

# misc
npm-debug.log*
yarn-error.log*
.env
```

### Yarn

```
.yarn/*
!.yarn/releases
!.yarn/patches
!.yarn/plugins
!.yarn/sdks
!.yarn/versions

!.yarn/cache
```

### Gradle

```
# Gradle files
.gradle
/build/

# IntelliJ IDEA
.idea/

# Eclipse
.classpath
.project
.settings/

# Visual Studio Code
.vscode/

# Mac OS
.DS_Store

# Windows
Thumbs.db

# Gradle wrapper
gradle/wrapper/gradle-wrapper.jar
gradle/wrapper/gradle-wrapper.properties

# Local Gradle properties
gradle.properties

# Gradle cache directory
.gradle/
```

### Maven

```
# Maven target directory
/target/

# Eclipse
.classpath
.project
.settings/

# IntelliJ IDEA
.idea/

# NetBeans
nbproject/

# VS Code
.vscode/

# Mac OS
.DS_Store

# Windows
Thumbs.db
```

### Webpack

```
# Logs
logs
*.log

# Dependency directories
node_modules/
bower_components/

# Production build
dist/

# Temporary files
.temp/
.temp-*

# Editor directories and files
.idea/
.vscode/
*.suo
*.ntvs*

# OS-specific files
.DS_Store
Thumbs.db
```

### Composer

```
composer.phar
/vendor/
```

### pip

```
# Python bytecode
__pycache__/

# Directory containing the project environment
env/

# Compiled Python files
*.pyc

# pipenv environment file
Pipfile.lock

# Jupyter Notebook checkpoints
.ipynb_checkpoints/

# Visual Studio Code
.vscode/

# macOS
.DS_Store

# Windows
Thumbs.db

# PyCharm files
.idea/
*.iml
*.iws
*.ipr
```

___

## Database management systems

### MySQL

```
# MySQL log files
*.log

# MySQL data directory (if not using Docker or other containerization)
/data/

# MySQL configuration files
/my.cnf
/my.ini

# MySQL temporary files
*.pid
*.err

# MySQL socket file
mysql.sock

# MySQL binary logs
*.index
*.log.0*
*.log.[0-9]*

# MySQL auto generated files
auto.cnf

# MySQL transaction logs
*.relay
*.relay-log*
```

### MongoDB

```
# MongoDB data directory
/data/

# MongoDB log files
*.log
```

### SQLite

```
# SQLite database file
*.sqlite

# SQLite database directory (if not using Docker or other containerization)
/db/

# SQLite temporary files
*.sqlite-journal
*.sqlite-shm
*.sqlite-wal
```

### Firebase

```
# Firebase local development
.firebase/
.firebaserc
.firebaserc

# Firebase SDK automatically created files
firebase.json

# Firebase Cloud Functions dependencies
functions/node_modules

# Firebase Firestore and Realtime Database backup files
firestore-backups/
```

___

## IDE and code editors

### VS Code

```
.vscode/*
!.vscode/settings.json
!.vscode/tasks.json
!.vscode/launch.json
!.vscode/extensions.json
!.vscode/*.code-snippets

.history/

*.vsix
```

### JetBrains IDE's

```
# IntelliJ IDEA
.idea/

# File-based project format
*.ipr
*.iws
*.iml

# Directory-based project format
.idea/

# SonarLint plugin
.sonarlint/

# User-specific files
*.suo
*.user
*.userosscache
```

### Visual Studio

```
*.rsuser
*.suo
*.user
*.userosscache
*.sln.docstates

*.userprefs

mono_crash.*

[Dd]ebug/
[Dd]ebugPublic/
[Rr]elease/
[Rr]eleases/
x64/
x86/
[Ww][Ii][Nn]32/
[Aa][Rr][Mm]/
[Aa][Rr][Mm]64/
bld/
[Bb]in/
[Oo]bj/
[Ll]og/
[Ll]ogs/

.vs/

Generated\ Files/

[Tt]est[Rr]esult*/
[Bb]uild[Ll]og.*

*.VisualState.xml
TestResult.xml
nunit-*.xml

[Dd]ebugPS/
[Rr]eleasePS/
dlldata.c

BenchmarkDotNet.Artifacts/

project.lock.json
project.fragment.lock.json
artifacts/

ScaffoldingReadMe.txt

StyleCopReport.xml

*_i.c
*_p.c
*_h.h
*.ilk
*.meta
*.obj
*.iobj
*.pch
*.pdb
*.ipdb
*.pgc
*.pgd
*.rsp
*.sbr
*.tlb
*.tli
*.tlh
*.tmp
*.tmp_proj
*_wpftmp.csproj
*.log
*.tlog
*.vspscc
*.vssscc
.builds
*.pidb
*.svclog
*.scc

_Chutzpah*

ipch/
*.aps
*.ncb
*.opendb
*.opensdf
*.sdf
*.cachefile
*.VC.db
*.VC.VC.opendb

*.psess
*.vsp
*.vspx
*.sap

*.e2e

$tf/

*.gpState

_ReSharper*/
*.[Rr]e[Ss]harper
*.DotSettings.user

_TeamCity*

*.dotCover

.axoCover/*
!.axoCover/settings.json

coverage*.json
coverage*.xml
coverage*.info

*.coverage
*.coveragexml

_NCrunch_*
.*crunch*.local.xml
nCrunchTemp_*

*.mm.*
AutoTest.Net/

.sass-cache/

[Ee]xpress/

DocProject/buildhelp/
DocProject/Help/*.HxT
DocProject/Help/*.HxC
DocProject/Help/*.hhc
DocProject/Help/*.hhk
DocProject/Help/*.hhp
DocProject/Help/Html2
DocProject/Help/html

publish/

*.[Pp]ublish.xml
*.azurePubxml
*.pubxml
*.publishproj

PublishScripts/

*.nupkg
*.snupkg
**/[Pp]ackages/*
!**/[Pp]ackages/build/
#!**/[Pp]ackages/repositories.config
*.nuget.props
*.nuget.targets

csx/
*.build.csdef

ecf/
rcf/

AppPackages/
BundleArtifacts/
Package.StoreAssociation.xml
_pkginfo.txt
*.appx
*.appxbundle
*.appxupload

*.[Cc]ache
!?*.[Cc]ache/

ClientBin/
~$*
*~
*.dbmdl
*.dbproj.schemaview
*.jfm
*.pfx
*.publishsettings
orleans.codegen.cs

Generated_Code/

_UpgradeReport_Files/
Backup*/
UpgradeLog*.XML
UpgradeLog*.htm
ServiceFabricBackup/
*.rptproj.bak

*.mdf
*.ldf
*.ndf

*.rdl.data
*.bim.layout
*.bim_*.settings
*.rptproj.rsuser
*- [Bb]ackup.rdl
*- [Bb]ackup ([0-9]).rdl
*- [Bb]ackup ([0-9][0-9]).rdl

FakesAssemblies/

*.GhostDoc.xml

.ntvs_analysis.dat
node_modules/

*.plg

*.opt

*.vbw

*.vbp

*.dsw
*.dsp

*.ncb
*.aps

**/*.HTMLClient/GeneratedArtifacts
**/*.DesktopClient/GeneratedArtifacts
**/*.DesktopClient/ModelManifest.xml
**/*.Server/GeneratedArtifacts
**/*.Server/ModelManifest.xml
_Pvt_Extensions

.paket/paket.exe
paket-files/

.fake/

.cr/personal

__pycache__/
*.pyc

*.tss

*.jmconfig

*.btp.cs
*.btm.cs
*.odx.cs
*.xsd.cs

OpenCover/

ASALocalRun/

*.binlog

*.nvuser

.mfractor/

.localhistory/

.vshistory/

healthchecksdb

MigrationBackup/

.ionide/

FodyWeavers.xsd

.vscode/*
!.vscode/settings.json
!.vscode/tasks.json
!.vscode/launch.json
!.vscode/extensions.json
*.code-workspace

.history/

*.cab
*.msi
*.msix
*.msm
*.msp

*.sln.iml
```

### Eclipse

```
.metadata
bin/
tmp/
*.tmp
*.bak
*.swp
*~.nib
local.properties
.settings/
.loadpath
.recommenders

.externalToolBuilders/

*.launch

*.pydevproject

.cproject

.autotools

.factorypath

.buildpath

.target

.tern-project

.texlipse

.springBeans

.recommenders/

.apt_generated/
.apt_generated_test/

.cache-main
.scala_dependencies
.worksheet
```

### Android Studio

```
# Android Studio
.idea/

# Gradle files
*.iml
.gradle/
/local.properties

# Proguard folder generated by Android Studio
proguard/

# Logs
*.log

# Android Studio Navigation editor temp files
.navigation/

# Android Studio captures folder (screenshots, etc.)
captures/

# Android Studio linter
.lint/

# Android Studio External Tools
.externalNativeBuild/

# Windows image file caches
Thumbs.db

# macOS directory settings
.DS_Store

# Android Studio workspace files
android-workspace.xml

# Android Studio user-specific files
*.suo
*.user
*.userosscache
```

### Xcode

```
xcuserdata/
```

### Sublime Text

```
*.tmlanguage.cache
*.tmPreferences.cache
*.stTheme.cache

*.sublime-workspace

sftp-config.json
sftp-config-alt*.json

Package Control.last-run
Package Control.ca-list
Package Control.ca-bundle
Package Control.system-ca-bundle
Package Control.cache/
Package Control.ca-certs/
Package Control.merged-ca-bundle
Package Control.user-ca-bundle
oscrypto-ca-bundle.crt
bh_unicode_properties.cache

GitHub.sublime-settings
```

### Atom

```
# Atom
.idea/
*.iml
.idea_modules/

# Node dependencies
node_modules/

# Compiled source
dist/

# Logs
logs/
*.log

# OS-specific files
.DS_Store
Thumbs.db
```

### Vim

```
[._]*.s[a-v][a-z]
!*.svg
[._]*.sw[a-p]
[._]s[a-rt-v][a-z]
[._]ss[a-gi-z]
[._]sw[a-p]

Session.vim
Sessionx.vim

.netrwhist
*~
tags
[._]*.un~
```

___

## Operating systems

### Windows

```
Thumbs.db
Thumbs.db:encryptable
ehthumbs.db
ehthumbs_vista.db

*.stackdump

[Dd]esktop.ini

$RECYCLE.BIN/

*.cab
*.msi
*.msix
*.msm
*.msp

*.lnk
```

### Linux

```
*~

.fuse_hidden*

.directory

.Trash-*

.nfs*
```

### MacOS

```
.DS_Store
.AppleDouble
.LSOverride

Icon

._*

.DocumentRevisions-V100
.fseventsd
.Spotlight-V100
.TemporaryItems
.Trashes
.VolumeIcon.icns
.com.apple.timemachine.donotpresent

.AppleDB
.AppleDesktop
Network Trash Folder
Temporary Items
.apdisk
```

___

## Other

### Sass/SCSS

```
.sass-cache/
*.css.map
*.sass.map
*.scss.map
```

### Less

```
*.less
```

### Jupyter Notebooks

```
.ipynb_checkpoints
*/.ipynb_checkpoints/*

profile_default/
ipython_config.py
```

### Terraform

```
**/.terraform/*

*.tfstate
*.tfstate.*

crash.log
crash.*.log

*.tfvars
*.tfvars.json

override.tf
override.tf.json
*_override.tf
*_override.tf.json

.terraformrc
terraform.rc
```