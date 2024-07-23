# Discord Auto Update for Linux
[한국어](#리눅스에서-디스코드를-다시-업데이트하기-귀찮으신가요) | [English](#tired-of-manually-updating-discord-on-linux)

저자: [김수현](https://github.com/ppsrac)

Author: [Suhyun Kim](https://github.com/ppsrac)

# 리눅스에서 디스코드를 수동으로 업데이트하기 귀찮으신가요?

그것이 귀찮아서 이 프로젝트를 만들었습니다. `install.sh`파일을 실행하면 앞으로는 귀찮게 `.deb`파일을 받을 필요 없이 부팅때마다 자동으로 업데이트를 할것입니다. 

## 설치 방법

1. git clone을 한다. 
2. cmd 창에 `./install.sh`를 실행한다. 
3. sudo 비밀번호를 입력한다. 
4. 끝

비밀번호는 `systemd` 대몬에 해당 `.service`파일을 추가하기 위해 필요하며
자동으로 업데이트할 시 `dpkg -i` 명령어를 사용하기 위해 필요합니다. 

## 주의할 점

위 shell file을 실행하게 되면 `/etc/systemd/system/discord_auto_update.service`
에 sudo 비밀번호가 저장됩니다.

sudo에 대한 비밀번호를 `/etc/systemd/system/discord_auto_update.service`에 저장하는
것이 위험할 수 있습니다. 때문에, 이를 원하시지 않는 분은 `install.sh` 파일을 직접 보시고
참고하여 수정하시기 바랍니다. 

기본적으로 현 repo의 `./discord_auto_update`는 rust로 만들어진 컴파일 된 실행파일이고
`./discord_auto_update your_sudo_password`을 통해 업데이트를 실행하게 할 수 있습니다. 

만일 해당 레포에서 궁금한 점이 있다면, 이슈를 남겨주시기 바랍니다.
_____

# Tired of Manually Updating Discord on Linux?
The author found it cumbersome and thus created this project for simplicity. By executing the `install.sh` file, you will no longer need to manually download `.deb` files, as updates will automatically be applied at each system boot.

## How to Install
1. Perform a git clone.
2. Run `./install.sh` in the command window.
3. Enter the sudo password.
4. Done.

The password is needed to add the `.service` file to the systemd daemon and
for using the `dpkg -i` command for automatic updates.

## Points of Caution

Running the above shell file will store your sudo password in `/etc/systemd/system/discord_auto_update.service`.

Storing your sudo password in `/etc/systemd/system/discord_auto_update.service` can be risky. 
Therefore, if you do not prefer this, it's advisable to directly examine the `install.sh` file and make modifications as needed.

By default, the `./discord_auto_update present` in this repo is a compiled executable made with Rust, 
and you can execute updates via `./discord_auto_update your_sudo_password`.

If you have any questions about this repo, please leave an issue.
