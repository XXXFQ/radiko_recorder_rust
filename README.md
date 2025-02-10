# Radiko Recorder Rust

このツールは、Radiko からラジオ放送を録音するための **Rust 製** アプリケーションです。  
Rust の高速性と安全性を活かして実装されており、シンプルなコマンドラインインターフェースで操作できます。

## 必要条件

このツールを使用する前に、以下の環境が整っていることを確認してください：

- **Rust ツールチェーン**  
  最新版の Rust を [公式サイト](https://www.rust-lang.org/tools/install) からインストールしてください。

- **ffmpeg のインストール**  
  録音機能には `ffmpeg` が必要です。`ffmpeg` がインストールされていない場合、以下の手順でインストールしてください：

  **Ubuntu/Debian**:
  ```sh
  sudo apt update
  sudo apt install ffmpeg
  ```

  **macOS (Homebrew)**:
  ```sh
  brew install ffmpeg
  ```

  **Windows**:
  1. [ffmpeg 公式サイト](https://ffmpeg.org/download.html) からバイナリをダウンロードしてください。
  2. ダウンロードした ffmpeg のパスをシステム環境変数に追加してください。

## 使い方

このツールは、放送局リストの表示とラジオ放送の録音の 2 つのモードで動作します。

### 放送局リストの表示

放送局リストを表示するには、`--station-list` オプションを利用します。

```sh
radiko_recorder_rust --station-list
```

### 放送局の録音

指定した放送局からラジオ放送を録音するには、以下の形式でコマンドを実行します。

```sh
radiko_recorder_rust <station_id> <start_time> <duration_minutes>
```

- `<station_id>`: 録音対象の放送局の ID (例: `TBS`, `QRR` など)  
- `<start_time>`: 録音開始時刻を `YYYYMMDDHHMMSS` 形式で指定  
- `<duration_minutes>`: 録音時間（分）

**例:**
```sh
radiko_recorder_rust FMT 20241120120000 50
```

上記の例では、2024年11月20日12:00:00 から 50 分間、TOKYO FM の放送を録音します。

## インストール方法

### GitHub からのクローンとビルド

以下の手順でリポジトリをクローンし、リリースビルドを行います。

```sh
git clone https://github.com/XXXFQ/radiko_recorder_rust.git
cd radiko_recorder_rust
cargo build --release
```

ビルドが完了すると、実行ファイルは `target/release/radiko_recorder_rust` に生成されます。

## ログ出力

このツールは、実行時に `logs` ディレクトリ内に日付別のログファイルを生成し、コンソールにも色付きでログを出力します。  
詳細なログはファイルとコンソールの両方で確認できます。

## ライセンス

本プロジェクトは [MIT ライセンス](./LICENSE) の下で提供されています。詳細は LICENSE ファイルをご確認ください。

## 著作権表示

© 2025 ARM