export JAVA_HOME=/opt/android-studio/jbr
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

npm run tauri android build -- --apk

cp src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk .

~/Android/Sdk/build-tools/36.0.0/apksigner sign \
	--ks ~/.android/debug.keystore \
	--ks-key-alias androiddebugkey \
	--ks-pass pass:android \
	--key-pass pass:android \
	--out RustyWords-signed.apk \
	app-universal-release-unsigned.apk

rm app-universal-release-unsigned.apk
