# pc88like_image
画像をPC8801風の画像に変換するアプリケーション。

## 使い方。
exec.batのパラメータを書き換えて実行する。

### exec.batの書き方
callの文は書き換えないでください
```batch
set importFile=D:/hogehoge/fugafuga.png
set exportFile=./piyopiyo.png
set gamma_sat=1
set gamma_light=1.5
call pc88_like_image.exe %importFile% %exportFile% %gamma_sat% %gamma_light%
```
パラメータの意味
* importFile: 入力ファイルパス
* exportFile: 出力ファイルパス
* gamma_sat: 変換時の彩度補正
* gamma_light: 変換時の明度補正

# 入出力の対応
入力(横幅640pxでのみ動作)
* pngフォーマット
* jpegフォーマット

出力
* pngフォーマット
* jpegフォーマット
JPEGの場合、品質100で出力します。
