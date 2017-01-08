# embed

Rustで実装したライブラリを外部モジュールとして他の言語から呼び出す。  

## Build
関数にno_mangle属性をつけてマングリングさせないようにし、pub externをつけておく。  
あとはCargo.tomlに下記を記述し、ダイナミックライブラリとする。  
```
[lib]
name = "embed"
crate-type = ["dylib"]
```

## Run
```
python embed.py
```
or
```
node embed.js
```

Result:
```
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
Thread finished with count=5000000
done!
```