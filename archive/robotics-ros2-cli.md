---
layout: archive
title: ROS2CLI
tags: [ROS2]
category: "RObotics"
---

ROS2 CLI를 구현에 사용되는 ros2cli 패키지에 대해 정리하고, ros2cli 패키지를 이용한 커스텀 cli를 만드는 예제를 정리한다.

# 1. ros2cli

[ros2cli](https://github.com/ros2/ros2cli)는 ROS2 command line interface 도구로써, 모든 ROS2 distro에 표준으로 사용되는 도구이다.

ros2 cli의 형태는 기본적으로 다음과 같다.

```bash
ros2 <command> <verb> --options
```

다음은 ros2cli에서 기본적으로 제공하는 명령어를 정리한 내용이다.

<img src="/assets/img/posts/230530_ros2cli_cheatsheet.png">

# 2. Add New Verbs

[python entry points](https://setuptools.pypa.io/en/latest/pkg_resources.html#entry-points)를 이용하여 새로운 Command와 Verb를 추가할 수 있다. 새로운 Command와 Verb를 추가한 예제로 [ros2hellocli](https://github.com/artivis/ros2hellocli)가 있다.

# 3. ros2hellocli

ros2hellocli에 대해 내용을 정리한 문서가 있다. [다음](https://ubuntu.com/blog/creating-a-ros-2-cli-command-and-verb)을 참조한다.

다음은 ros2hellocli의 디렉토리(필수적이지 않은 요소들은 제외한) 구조이다. ros2cli 패키지는 기본적으로 `api`, `command`, `verb`의 디렉토리 구조를 가지며, `package.xml`과 `setup.py`의 설정파일을 갖는다.

```bash
ros2hellocli
├── ros2hellocli
│   ├── api
│   │   └── __init__.py
│   ├── command
│   │   ├── hello.py
│   │   └── __init__.py
│   ├── verb
│   │   ├── __init__.py
│   │   └── world.py
│   └── __init__.py
├── package.xml
└── setup.py
```

우선 `package.xml`은 다음과 같이 다른 ROS2 패키지와 동일하게, 패키지 설정 파일을 작성하고, depend로 ros2cli를 추가한다. 그 이외에 추가 depend가 있는 경우 추가하도록 한다.

```xml
<?xml version="1.0"?>
<package format="2">
  <name>ros2hellocli</name>
  <version>0.0.0</version>
  <description>
    The ROS 2 command line tools example.
  </description>
  <maintainer email="jeremie.deray@example.org">Jeremie Deray</maintainer>
  <license>Apache License 2.0</license>

  <depend>ros2cli</depend>

  <export>
    <build_type>ament_python</build_type>
  </export>
</package>
```

그리고 `setup.py`는 ROS2 python 패키지와 동일하게 사용하며, 이제부터 나올 `command`와 `verb`에 대한 `entry_points`를 추가해나갈 것이다.

```python
from setuptools import find_packages
from setuptools import setup

setup(
  name='ros2hellocli',
  version='0.0.0',
  packages=find_packages(exclude=['test']),
  install_requires=['ros2cli'],
  zip_safe=True,
  author='Jeremie Deray',
  author_email='jeremie.deray@example.org',
  maintainer='Jeremie Deray',
  maintainer_email='jeremie.deray@example.org',
  url='https://github.com/artivis/ros2hellocli',
  download_url='https://github.com/artivis/ros2hellocli/releases',
  keywords=[],
  classifiers=[
      'Environment :: Console',
      'Intended Audience :: Developers',
      'License :: OSI Approved :: Apache Software License',
      'Programming Language :: Python',
  ],
  description='A minimal plugin example for ROS 2 command line tools.',
  long_description="""The package provides the hello command as a plugin example of ROS 2 command line tools.""",
  license='Apache License, Version 2.0',
)
```

## 3.1 command

`command`폴더는 ros2cli의 `<command>` 부분으로, ROS2 CLI 사용 시 새 명령을 검색할 수 있게 한다.

다음은 `hello.py` 코드이다.

```python
from ros2cli.command import add_subparsers_on_demand
from ros2cli.command import CommandExtension
from ros2cli.verb import get_verb_extensions

class HelloCommand(CommandExtension):
    """The 'hello' command extension."""

    def add_arguments(self, parser, cli_name):
        self._subparser = parser
        add_subparsers_on_demand(
            parser, cli_name, '_verb', 'ros2hellocli.verb', required=False)

    def main(self, *, parser, args):
        if not hasattr(args, '_verb'):
            self._subparser.print_help()
            return 0

        extension = getattr(args, '_verb')

        return extension.main(args=args)
```
* 원래 `hello.py` 코드에는 `add_subparsers` 함수를 사용했는데, `add_subparsers_on_demand` 함수를 사용할 것을 권장하고 있어, `add_subparsers_on_demand` 함수로 변환한 코드를 가져왔다.

이렇게 구현된 `hello.py`는 `entry_points`에 추가함으로써 `command`에 대한 새 진입점을 알릴 수 있다.

```python
  ...
  tests_require=['pytest'],
  entry_points={
        'ros2cli.command': [
            'hello = ros2hellocli.command.hello:HelloCommand',
        ]
    }
)
```
* hello라는 command가 되었다.

## 3.2 verb

`verb`폴더는 `<command>` 다음에 오는 `<verb>`의 역할을 하는 확장점으로, 실제 기능 구현을 하는 동사 확장점이 된다.

다음은 `world.py` 코드이다.

```python
from ros2cli.verb import VerbExtension


class WorldVerb(VerbExtension):
    """Prints Hello World on the terminal."""

    def main(self, *, args):
      print('Hello, ROS 2 World!')
```

이렇게 구현된 `world.py` 또한 `hello.py`와 같이 `setup.py`의 `entry_points`에 추가함으로써 `verb`에 대한 새 진입점을 알릴 수 있다.

```python
  ...
  tests_require=['pytest'],
  entry_points={
        'ros2cli.command': [
            'hello = ros2hellocli.command.hello:HelloCommand',
        ],
        'ros2hellocli.verb': [
            'world = ros2hellocli.verb.world:WorldVerb',
        ]
    }
)
```

만약, `world.py` 이외에도 `rootb.py`를 만든다면, `setup.py`의 `entry_points`에 추가하여 새로운 `verb`를 추가할 수 있다.

```python
...
  tests_require=['pytest'],
  entry_points={
        'ros2cli.command': [
            'hello = ros2hellocli.command.hello:HelloCommand',
        ],
        'ros2hellocli.verb': [
            'world = ros2hellocli.verb.world:WorldVerb',
            'rootb = ros2hellocli.verb.rootb:RootbVerb',
        ]
    }
)
```

## 3.3 api

이제 `command`와 `verb`를 구현하였으니, 사용자의 입력 인수를 처리하기 위하여 `api`에 기능을 추가할 것이다.

다음은 `api/__init__.py` 코드이다.

```python
def get_hello_world():
    return 'Hello, ROS 2 World!'

def get_hello_world_leet():
    return 'He110, R0S 2 W04ld!'
```
* print할 구문을 호출하는 두 개의 함수가 구현되어 있다.
* 위의 함수를 `verb class`에서 활용할 수 있도록 `api`에서 제공하는 것인데, 이 부분은 필수적인 부분은 아니지만, 일반적으로 콘솔창의 출력을 만들어 내기 위한 도구로 많이 구현된다.

이렇게 구현된 함수를 이제 `verb`인 `world.py`에서 사용하도록, 그리고 사용자의 입력 인수를 처리하도록 다음과 같이 확장한다.

```python
from ros2cli.verb import VerbExtension
from ros2hellocli.api import get_hello_world, get_hello_world_leet


class WorldVerb(VerbExtension):
    """Prints Hello World on the terminal."""

    def add_arguments(self, parser, cli_name):
      parser.add_argument(
                  '--leet', '-l', action='store_true',
                  help="Display the message in 'l33t' form.")

    def main(self, *, args):
      message = get_hello_world() if not args.leet else get_hello_world_leet()
      print(message)
```
* add_arguments 함수에서 --leet이나 -l인수를 받을 수 있도록 한다.
* 사용자의 입력 인수를 확인하여 사용할 함수를 결정하고 출력한다.

위와 같이 `api`에 정의된 함수 또는 다른 모듈에서 제공되는 `api`를 이용하여 사용자가 입력한 `verb`와 입력 인수에 따라 행동을 만들어낼 수 있을 것이다.

> cli의 옵션을 넣거나, argument의 completer 기능 등은 ros2cli 패키지에 구현되어 있는 ros2service, ros2topic 등과 같은 패키지 또는 [ros2_control](https://github.com/ros-controls/ros2_control)의 ros2controlcli에 잘 구현되어있으니 실제 구현 시 참고하여 만들면 좋다.

## 3.4 완성된 cli의 실행

구현이 완료된 ros2hellocli 패키지를 빌드하면, 다음과 같이 터미널에서 명령을 할 수 있게 된다.

```bash
ros2 hello world
ros2 hello world --leet 
```

# 참고문헌

- [ros2cli repo](https://github.com/ros2/ros2cli)
- [ros2cli example ros2hellocli](https://github.com/artivis/ros2hellocli)
- [Creating a ROS 2 CLI command and verb](https://ubuntu.com/blog/creating-a-ros-2-cli-command-and-verb)
- [ros2_control repo](https://github.com/ros-controls/ros2_control)
- [ros2_control cli](https://control.ros.org/master/doc/ros2_control/ros2controlcli/doc/userdoc.html)