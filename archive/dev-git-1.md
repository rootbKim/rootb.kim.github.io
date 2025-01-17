---
layout: archive
title: Git이란?
tags: [Git]
category: "Dev"
---

버전관리로 기본적으로 가장 많이 사용되는 Git에 대해 간략하게 소개한다.

# 1. Git이란?

Git에 대한 Git 홈페이지 설명은 다음과 같다.

> Git은 소규모 프로젝트부터 초대형 프로젝트까지 모든 것을 빠르고 효율적으로 처리할 수 있도록 설계된 자유-오픈 소스 분산 버전 제어 시스템이다. Git은 배우기 쉽고 작은 풋프린트에 번개처럼 빠른 성능을 발휘합니다. Subversion, CVS, Perforce 및 ClearCase와 같은 SCM 툴보다 저렴한 로컬 분기, 편리한 스테이징 영역 및 여러 워크플로우가 제공됩니다.

# 2. 버전 관리

버전관리(VCS - Version Control System)란 파일 변화를 시간에 따라 기록했다가 나중에 특정 시점의 버전을 다시 꺼내올 수 있는 시스템을 말한다.

VCS를 사용하면 각 파일을 이전 상태로 되돌릴 수 있고, 프로젝트를 통째로 이전 상태로 되돌릴 수 있고, 시간에 따라 수정 내용을 비교해 볼 수 있고, 누가 문제를 일으켰는지도 추적할 수 있고, 누가 언제 만들어낸 이슈인지도 알 수 있다. VCS를 사용하면 파일을 잃어버리거나 잘못 고쳤을 때도 쉽게 복구할 수 있다.

## 2.1 로컬 버전 관리

로컬 VCS는 아주 간단한 데이터베이스를 사용해서 파일의 변경 정보를 관리한다. 많이 쓰는 VCS 도구 중에 RCS(Revision Control System)라고 부르는 것이 있는데, RCS는 기본적으로 Patch Set(파일에서 변경되는 부분)을 관리한다. 이 Patch Set은 특별한 형식의 파일로 저장하고, Patch Set을 적용해서 모든 파일을 특정 시점으로 되돌릴 수 있다.

## 2.2 중앙집중식 버전 관리(CVCS)

다른 개발자와 함께 작업해야 하는 경우 생기는 문제를 해결하기 위해 CVCS(중앙집중식 VCS)가 개발됐다. CVS, Subversion, Perforce 같은 시스템은 파일을 관리하는 서버가 별도로 있고 클라이언트가 중앙 서버에서 파일을 받아서 사용(Checkout)한다.

이 CVCS 환경의 가장 대표적인 문제는 서버가 다운되면 그동안 아무도 다른 사람과 협업할 수 없고 사람들이 하는 일을 백업할 방법도 없다. 그리고 중앙 데이터베이스가 있는 하드디스크에 문제가 생기면 프로젝트의 모든 히스토리를 잃게되는 문제가 있다.

## 2.3 분산 버전 관리(DVCS)

Git, Mecurial, Bazaar, Darcs 같은 DVCS에서의 클라이언트는 단순히 파일의 마지막 스냅샷을 Checkout 하는 것이 아니라, 저장소를 히스토리와 더불어 전부 복제한다. 이는 서버에 문제가 생기면 이 복제물로 다시 작업을 시작할 수 있게 한다. Clone은 모든 데이터를 가진 진정한 백업이다.

또한 DVCS 환경에서는 리모트 저장소가 존재한다. 리모트 저장소가 많을 수도 있다. 그래서 사람들은 동시에 다양한 그룹과 다양한 방법으로 협업할 수 있는 장점을 가진다.

# 3. 세 가지 상태

Git은 파일을 Committed, Modified, Staged 이렇게 세 가지 상태로 관리한다.

- Committed : 데이터가 로컬 데이터베이스에 안전하게 저장됐다는 것을 의미한다.
- Modified : 수정한 파일을 아직 로컬 데이터베이스에 커밋하지 않은 것을 말한다.
- Staged : 현재 수정한 파일을 곧 커밋할 것이라고 표시한 상태를 의미한다.

이 세 가지 상태는 Git 프로젝트의 세 가지 단계와 연결돼 있다. Git 디렉토리, 워킹 트리, Staging Area 이렇게 세 가지 단계를 이해하고 넘어가자.

<img src="/assets/img/posts/230105_areas.png">

- Git 디렉토리 : Git이 프로젝트의 메타데이터와 객체 데이터베이스를 저장하는 곳을 말한다. 이 Git 디렉토리가 Git의 핵심이다. 다른 컴퓨터에 있는 저장소를 Clone 할 때 Git 디렉토리가 만들어진다.
- 워킹 트리 : 프로젝트의 특정 버전을 Checkout 한 것이다.
- Staging Area : 곧 커밋할 파일에 대한 정보를 저장한다. Git에서는 기술용어로는 Index 라고 하지만, Staging Area 라는 용어를 써도 상관 없다.

이에 따른 git의 작업은 다음의 세 단계를 거친다.

- 워킹 트리에서 파일을 수정한다.
- Staging Area에 파일을 Stage 해서 커밋할 스냅샷을 만든다. 모든 파일을 추가할 수도 있고 선택하여 추가할 수도 있다.
- Staging Area에 있는 파일들을 커밋해서 Git 디렉토리에 영구적인 스냅샷으로 저장한다.

# 4. Git 매뉴얼

Git에 대한 매뉴얼은 Git에서 제공하는 다음 ['Pro Git'](https://git-scm.com/book/ko/v2)에서 참조한다.

# 참고문헌

- [Git 페이지](https://git-scm.com/)
- [Pro Git](https://git-scm.com/book/ko/v2)
- [Pro Git-버전 관리란?](https://git-scm.com/book/ko/v2/%EC%8B%9C%EC%9E%91%ED%95%98%EA%B8%B0-%EB%B2%84%EC%A0%84-%EA%B4%80%EB%A6%AC%EB%9E%80%3F)
- [Pro Git-Git 기초](https://git-scm.com/book/ko/v2/%EC%8B%9C%EC%9E%91%ED%95%98%EA%B8%B0-Git-%EA%B8%B0%EC%B4%88)
