# Senior Project

## What is this?

卒業研究のリポジトリ。

遺伝的アルゴリズムにより、spirv-optの最適な最適化適用順序を決定する。

## Build

### Windows

1. MSVCインストール
2. Rustインストール
3. VulkanSDKインストール
4. 環境変数`VulkanInclude`を`VulkanSDK/<version>/Include`に設定
5. 環境変数`VulkanLib`を`VulkanSDK/<version>/Lib`に設定
6. `external/setup.bat`を実行
7. `build-shader.bat`を実行
8. `build-ga.bat`を実行
9. `bin/ga.exe`を実行
