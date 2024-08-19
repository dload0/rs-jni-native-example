# rs-jni-native-example
> A JNI native example in Rust. 

<p align='left'>
    <img src="https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324" />
    <img src="https://img.shields.io/badge/Java-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white" />
</p>

| Folder | Description |
| --- | --- |
| `native-loader/` | The Java program responsible for loading the native, and calling a function. |
| `root` | The Rust native code. |

## Build


Run the command in the project root to build the Rust native.

```
cargo build --release
```

Run the command in the `native-loader` folder to build the Java program.

```
./gradlew build
```
