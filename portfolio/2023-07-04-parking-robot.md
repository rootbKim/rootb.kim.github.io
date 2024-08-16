---
layout: post
title: 저상형 주차로봇의 서비스 관리
feature-img: "/assets/img/portfolio/parking/parking.jpg"
img: "/assets/img/portfolio/parking/parking.jpg"
date: 04 July 2023
excerpt: 저상형 주차로봇의 서비스 상태를 관리하는 서비스 관리 모듈을 개발하고, 두 대의 저상형 로봇에 대한 협조 제어를 수행한 프로젝트이다.
---

# 개요

저상형 주차로봇의 미션 및 동작의 상태를 관리하는 서비스 관리 모듈을 개발하고, 두 대의 로봇의 협조 제어를 위한 모션 제어 방법을 공동으로 수행한 프로젝트이다.

# 프로젝트 기간

2023-07-04 ~ 2023-08-24

# 개발 참여 인원 / 담당 역할

6명 / 서비스 관리자 개발 및 협조 제어

# 기술 스택

- ROS2(galactic)
- C++
- pyqt
- Motion Control

# 세부 내용

### 서비스 관리자 개발

* 로봇의 움직임 상태, 동작의 상태 등 관제 서버에서 명령한 내용에 대한 수행 상태를 관리하도록 하였다.
* 관제 서버에서의 명령을 로봇이 수행할 수 있는지 여부를 판단하고, 해당하는 명령을 로봇의 구동 모듈에 전달하여 로봇이 미션을 수행하도록 개발하였다.
* 로봇 간의 TCP/IP 연결 상태를 관리하고, Pairing, Unpairing을 관리하였다.

### 테스트 UI 개발

* 개발된 서비스 관리자의 기능을 검증하기 위하여, pyqt를 이용하여 ROS2 인터페이스를 테스트할 수 있는 UI 툴을 개발하였다.
* 개발된 UI는 관제 서버 없이 단위 기능을 테스트할 수 있는 툴로써 사용되었다.

### 협조 모션 방법

* 로봇 간의 제자리 회전 협조 모션을 수행하기 위한 PD 제어의 방법을 고도화하였다.
* 로봇의 구동부의 구조상 Singularity가 걸리는 구조인데, 이에 대한 해결 방안을 제시하였다.
* GAZEBO 시뮬레이션과, 실 로봇 테스트에서 성공적으로 제자리 회전 모션을 수행할 수 있도록 하였다.

### Conclusion

* 로봇의 상태를 관리하고, 미션을 원할하게 수행할 수 있도록 하는 서비스 모듈을 개발할 수 있었다.
* 로봇의 상태를 관리하기 위하여 Behavior Tree를 이용하는 개발 방법도 생각해 볼 수 있었다.
* pyqt를 이용하여 UI를 만들 수 있었고, 이 과정에서 pyqt에서 ROS2 인터페이스를 사용할 수 있도록 프로그램을 구성하였다.
* 모바일 로봇의 모션 제어 방법에 대해 고민할 수 있었고, 로봇 간의 협조 제어를 위한 방법에 대해서도 고민할 수 있었다.