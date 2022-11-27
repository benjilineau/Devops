# Devops

Serveur web développé en rust en utilisant le moins de dépendances possible. Le serveur est une API qui renvoit les headers d'une requête sur la route /ping (HTTP GET)). Toutes les autres routes renvoient vers une erreur 404 Le port d'écoute du serveur est modifiable. (variable PING_LISTEN_PORT).

## lancer le projet

- Dans un premier temps intaller rust
- Ensuite, il faut cloner le projet `git clone https://github.com/benjilineau/Devops.git`
- Lancer le serveur avec la commande : `cargo run`
- Rendez-vous maintenant dans votre navigateur a l'adresse suivante: `http://localhost:8080/ping`. Si toutes les étapes ont été respécté alors vous devriez voir le contenu de votre header sur la page.

docker run --rm -i -t -p 8080:8080 tp2devops

### Scan

```
Benjamin@DESKTOP-ON7BJGL MINGW64 ~/Documents/ynov B3/Devops/WIK-DPS-TP02 (main)
$ docker scan --file dockerfile tp2devops
- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container dependencies for tp2de| Analyzing container dependencies for tp2de/ Analyzing container dependencies for tp2de- Analyzing container dependencies for tp2de\ Analyzing container depend

Testing tp2devops...

✗ Low severity vulnerability found in wget
  Description: Open Redirect
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-WGET-1277610
  Introduced through: wget@1.21-1+deb11u1
  From: wget@1.21-1+deb11u1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in util-linux/libblkid1
  Description: Information Exposure
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-UTILLINUX-2401081
  Introduced through: util-linux/libblkid1@2.36.1-8+deb11u1, e2fsprogs@1.46.2-2, util-linux/libmount1@2.36.1-8+deb11u1, util-linux/mount@2.36.1-8+deb11u1, wget@1.21-1+deb11u1, util-linux@2.36.1-8+deb11u1, util-linux/bsdutils@1:2.36.1-8+deb11u1, util-linux/libsmartcols1@2.36.1-8+deb11u1
  From: util-linux/libblkid1@2.36.1-8+deb11u1
  From: e2fsprogs@1.46.2-2 > util-linux/libblkid1@2.36.1-8+deb11u1
  From: util-linux/libmount1@2.36.1-8+deb11u1 > util-linux/libblkid1@2.36.1-8+deb11u1
  and 15 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in tar
  Description: CVE-2005-2541
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-TAR-523480
  Introduced through: meta-common-packages@meta
  From: meta-common-packages@meta > tar@1.34+dfsg-1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in systemd/libsystemd0
  Description: Authentication Bypass
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-SYSTEMD-1291054
  Introduced through: systemd/libsystemd0@247.3-7+deb11u1, apt@2.2.4, util-linux/bsdutils@1:2.36.1-8+deb11u1, util-linux/mount@2.36.1-8+deb11u1, systemd/libudev1@247.3-7+deb11u1
  From: systemd/libsystemd0@247.3-7+deb11u1
  From: apt@2.2.4 > systemd/libsystemd0@247.3-7+deb11u1
  From: util-linux/bsdutils@1:2.36.1-8+deb11u1 > systemd/libsystemd0@247.3-7+deb11u1
  and 5 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in systemd/libsystemd0
  Description: Off-by-one Error
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-SYSTEMD-3111119
  Introduced through: systemd/libsystemd0@247.3-7+deb11u1, apt@2.2.4, util-linux/bsdutils@1:2.36.1-8+deb11u1, util-linux/mount@2.36.1-8+deb11u1, systemd/libudev1@247.3-7+deb11u1
  From: systemd/libsystemd0@247.3-7+deb11u1
  From: apt@2.2.4 > systemd/libsystemd0@247.3-7+deb11u1
  From: util-linux/bsdutils@1:2.36.1-8+deb11u1 > systemd/libsystemd0@247.3-7+deb11u1
  and 5 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in systemd/libsystemd0
  Description: Link Following
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-SYSTEMD-524969
  Introduced through: systemd/libsystemd0@247.3-7+deb11u1, apt@2.2.4, util-linux/bsdutils@1:2.36.1-8+deb11u1, util-linux/mount@2.36.1-8+deb11u1, systemd/libudev1@247.3-7+deb11u1
  From: systemd/libsystemd0@247.3-7+deb11u1
  From: apt@2.2.4 > systemd/libsystemd0@247.3-7+deb11u1
  From: util-linux/bsdutils@1:2.36.1-8+deb11u1 > systemd/libsystemd0@247.3-7+deb11u1
  and 5 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in shadow/passwd
  Description: Access Restriction Bypass
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-SHADOW-526940
  Introduced through: shadow/passwd@1:4.8.1-1, adduser@3.118, shadow/login@1:4.8.1-1,
util-linux/mount@2.36.1-8+deb11u1
  From: shadow/passwd@1:4.8.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1
  From: shadow/login@1:4.8.1-1
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in shadow/passwd
  Description: Time-of-check Time-of-use (TOCTOU)
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-SHADOW-528840
  Introduced through: shadow/passwd@1:4.8.1-1, adduser@3.118, shadow/login@1:4.8.1-1,
util-linux/mount@2.36.1-8+deb11u1
  From: shadow/passwd@1:4.8.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1
  From: shadow/login@1:4.8.1-1
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in shadow/passwd
  Description: Incorrect Permission Assignment for Critical Resource
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-SHADOW-539870
  Introduced through: shadow/passwd@1:4.8.1-1, adduser@3.118, shadow/login@1:4.8.1-1,
util-linux/mount@2.36.1-8+deb11u1
  From: shadow/passwd@1:4.8.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1
  From: shadow/login@1:4.8.1-1
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in perl/perl-base
  Description: Improper Verification of Cryptographic Signature
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PERL-1925976
  Introduced through: meta-common-packages@meta
  From: meta-common-packages@meta > perl/perl-base@5.32.1-4+deb11u2
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in perl/perl-base
  Description: Link Following
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PERL-532614
  Introduced through: meta-common-packages@meta
  From: meta-common-packages@meta > perl/perl-base@5.32.1-4+deb11u2
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in pcre3/libpcre3
  Description: Out-of-Bounds
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PCRE3-523392
  Introduced through: pcre3/libpcre3@2:8.39-13, grep@3.6-1
  From: pcre3/libpcre3@2:8.39-13
  From: grep@3.6-1 > pcre3/libpcre3@2:8.39-13
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in pcre3/libpcre3
  Description: Out-of-Bounds
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PCRE3-525075
  Introduced through: pcre3/libpcre3@2:8.39-13, grep@3.6-1
  From: pcre3/libpcre3@2:8.39-13
  From: grep@3.6-1 > pcre3/libpcre3@2:8.39-13
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in pcre3/libpcre3
  Description: Uncontrolled Recursion
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PCRE3-529298
  Introduced through: pcre3/libpcre3@2:8.39-13, grep@3.6-1
  From: pcre3/libpcre3@2:8.39-13
  From: grep@3.6-1 > pcre3/libpcre3@2:8.39-13
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in pcre3/libpcre3
  Description: Out-of-Bounds
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PCRE3-529490
  Introduced through: pcre3/libpcre3@2:8.39-13, grep@3.6-1
  From: pcre3/libpcre3@2:8.39-13
  From: grep@3.6-1 > pcre3/libpcre3@2:8.39-13
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in pcre3/libpcre3
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-PCRE3-572353
  Introduced through: pcre3/libpcre3@2:8.39-13, grep@3.6-1
  From: pcre3/libpcre3@2:8.39-13
  From: grep@3.6-1 > pcre3/libpcre3@2:8.39-13
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in openssl/libssl1.1
  Description: Inadequate Encryption Strength
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-OPENSSL-2941242
  Introduced through: ca-certificates@20210119, glibc/libc6-dev@2.31-13+deb11u5
  From: ca-certificates@20210119 > openssl@1.1.1n-0+deb11u3 > openssl/libssl1.1@1.1.1n-0+deb11u3
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2 > openssl/libssl1.1@1.1.1n-0+deb11u3
  From: ca-certificates@20210119 > openssl@1.1.1n-0+deb11u3
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in openssl/libssl1.1
  Description: Use of a Broken or Risky Cryptographic Algorithm
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-OPENSSL-518334
  Introduced through: ca-certificates@20210119, glibc/libc6-dev@2.31-13+deb11u5
  From: ca-certificates@20210119 > openssl@1.1.1n-0+deb11u3 > openssl/libssl1.1@1.1.1n-0+deb11u3
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2 > openssl/libssl1.1@1.1.1n-0+deb11u3
  From: ca-certificates@20210119 > openssl@1.1.1n-0+deb11u3
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in openssl/libssl1.1
  Description: Cryptographic Issues
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-OPENSSL-525332
  Introduced through: ca-certificates@20210119, glibc/libc6-dev@2.31-13+deb11u5
  From: ca-certificates@20210119 > openssl@1.1.1n-0+deb11u3 > openssl/libssl1.1@1.1.1n-0+deb11u3
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2 > openssl/libssl1.1@1.1.1n-0+deb11u3
  From: ca-certificates@20210119 > openssl@1.1.1n-0+deb11u3
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in ncurses/libtinfo6
  Description: Out-of-bounds Write
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-NCURSES-1655741
  Introduced through: ncurses/libtinfo6@6.2+20201114-2, bash@5.1-2+deb11u1, ncurses/ncurses-bin@6.2+20201114-2, util-linux/mount@2.36.1-8+deb11u1, ncurses/ncurses-base@6.2+20201114-2
  From: ncurses/libtinfo6@6.2+20201114-2
  From: bash@5.1-2+deb11u1 > ncurses/libtinfo6@6.2+20201114-2
  From: ncurses/ncurses-bin@6.2+20201114-2 > ncurses/libtinfo6@6.2+20201114-2
  and 3 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in ncurses/libtinfo6
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-NCURSES-2767191
  Introduced through: ncurses/libtinfo6@6.2+20201114-2, bash@5.1-2+deb11u1, ncurses/ncurses-bin@6.2+20201114-2, util-linux/mount@2.36.1-8+deb11u1, ncurses/ncurses-base@6.2+20201114-2
  From: ncurses/libtinfo6@6.2+20201114-2
  From: bash@5.1-2+deb11u1 > ncurses/libtinfo6@6.2+20201114-2
  From: ncurses/ncurses-bin@6.2+20201114-2 > ncurses/libtinfo6@6.2+20201114-2
  and 3 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libtasn1-6
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBTASN16-3061097
  Introduced through: wget@1.21-1+deb11u1
  From: wget@1.21-1+deb11u1 > gnutls28/libgnutls30@3.7.1-5+deb11u2 > libtasn1-6@4.16.0-2
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libsepol/libsepol1
  Description: Use After Free
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBSEPOL-1315627
  Introduced through: libsepol/libsepol1@3.1-1, adduser@3.118
  From: libsepol/libsepol1@3.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1 > libsemanage/libsemanage1@3.1-1+b2 >
libsepol/libsepol1@3.1-1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libsepol/libsepol1
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBSEPOL-1315629
  Introduced through: libsepol/libsepol1@3.1-1, adduser@3.118
  From: libsepol/libsepol1@3.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1 > libsemanage/libsemanage1@3.1-1+b2 >
libsepol/libsepol1@3.1-1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libsepol/libsepol1
  Description: Use After Free
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBSEPOL-1315635
  Introduced through: libsepol/libsepol1@3.1-1, adduser@3.118
  From: libsepol/libsepol1@3.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1 > libsemanage/libsemanage1@3.1-1+b2 >
libsepol/libsepol1@3.1-1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libsepol/libsepol1
  Description: Use After Free
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBSEPOL-1315641
  Introduced through: libsepol/libsepol1@3.1-1, adduser@3.118
  From: libsepol/libsepol1@3.1-1
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1 > libsemanage/libsemanage1@3.1-1+b2 >
libsepol/libsepol1@3.1-1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libgcrypt20
  Description: Information Exposure
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBGCRYPT20-1297892
  Introduced through: libgcrypt20@1.8.7-6, apt@2.2.4
  From: libgcrypt20@1.8.7-6
  From: apt@2.2.4 > apt/libapt-pkg6.0@2.2.4 > libgcrypt20@1.8.7-6
  From: apt@2.2.4 > gnupg2/gpgv@2.2.27-2+deb11u2 > libgcrypt20@1.8.7-6
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in libgcrypt20
  Description: Use of a Broken or Risky Cryptographic Algorithm
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-LIBGCRYPT20-523947
  Introduced through: libgcrypt20@1.8.7-6, apt@2.2.4
  From: libgcrypt20@1.8.7-6
  From: apt@2.2.4 > apt/libapt-pkg6.0@2.2.4 > libgcrypt20@1.8.7-6
  From: apt@2.2.4 > gnupg2/gpgv@2.2.27-2+deb11u2 > libgcrypt20@1.8.7-6
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in krb5/libk5crypto3
  Description: CVE-2022-42898
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-KRB5-3120880
  Introduced through: glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libk5crypto3@1.18.3-6+deb11u2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2 > krb5/libk5crypto3@1.18.3-6+deb11u2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2
  and 2 more...
  Image layer: Introduced by your base image (rust:slim)
  Fixed in: 1.18.3-6+deb11u3

✗ Low severity vulnerability found in krb5/libk5crypto3
  Description: CVE-2004-0971
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-KRB5-519904
  Introduced through: glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libk5crypto3@1.18.3-6+deb11u2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2 > krb5/libk5crypto3@1.18.3-6+deb11u2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2
  and 2 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in krb5/libk5crypto3
  Description: Integer Overflow or Wraparound
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-KRB5-524883
  Introduced through: glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libk5crypto3@1.18.3-6+deb11u2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2 > krb5/libk5crypto3@1.18.3-6+deb11u2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > krb5/libkrb5-3@1.18.3-6+deb11u2
  and 2 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in gnutls28/libgnutls30
  Description: Improper Input Validation
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GNUTLS28-515971
  Introduced through: apt@2.2.4, wget@1.21-1+deb11u1
  From: apt@2.2.4 > gnutls28/libgnutls30@3.7.1-5+deb11u2
  From: wget@1.21-1+deb11u1 > gnutls28/libgnutls30@3.7.1-5+deb11u2
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: Out-of-Bounds
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-521063
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: Uncontrolled Recursion
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-521199
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: Use of Insufficiently Random Values
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-522385
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: Information Exposure
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-529848
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: CVE-2019-1010023
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-531451
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: Uncontrolled Recursion
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-531492
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in glibc/libc-bin
  Description: Resource Management Errors
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-GLIBC-532215
  Introduced through: glibc/libc-bin@2.31-13+deb11u5, glibc/libc6-dev@2.31-13+deb11u5, meta-common-packages@meta
  From: glibc/libc-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5 > glibc/libc-dev-bin@2.31-13+deb11u5
  From: glibc/libc6-dev@2.31-13+deb11u5
  and 1 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in e2fsprogs/libcom-err2
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-E2FSPROGS-2628459
  Introduced through: e2fsprogs@1.46.2-2, glibc/libc6-dev@2.31-13+deb11u5, e2fsprogs/libext2fs2@1.46.2-2, e2fsprogs/libss2@1.46.2-2, e2fsprogs/logsave@1.46.2-2
  From: e2fsprogs@1.46.2-2 > e2fsprogs/libcom-err2@1.46.2-2
  From: e2fsprogs@1.46.2-2 > e2fsprogs/libss2@1.46.2-2 > e2fsprogs/libcom-err2@1.46.2-2
  From: glibc/libc6-dev@2.31-13+deb11u5 > libnsl/libnsl-dev@1.3.0-2 > libnsl/libnsl2@1.3.0-2 > libtirpc/libtirpc3@1.3.1-1+deb11u1 > krb5/libgssapi-krb5-2@1.18.3-6+deb11u2 > e2fsprogs/libcom-err2@1.46.2-2
  and 8 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in db5.3/libdb5.3
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-DB53-2825168
  Introduced through: db5.3/libdb5.3@5.3.28+dfsg1-0.8, adduser@3.118
  From: db5.3/libdb5.3@5.3.28+dfsg1-0.8
  From: adduser@3.118 > shadow/passwd@1:4.8.1-1 > pam/libpam-modules@1.4.0-9+deb11u1 > db5.3/libdb5.3@5.3.28+dfsg1-0.8
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in coreutils/coreutils
  Description: Improper Input Validation
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-COREUTILS-514776
  Introduced through: coreutils/coreutils@8.32-4+b1
  From: coreutils/coreutils@8.32-4+b1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in coreutils/coreutils
  Description: Race Condition
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-COREUTILS-527269
  Introduced through: coreutils/coreutils@8.32-4+b1
  From: coreutils/coreutils@8.32-4+b1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-1054596
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Race Condition
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-1065551
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Write
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-1086496
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Improper Input Validation
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-1244572
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Uncontrolled Recursion
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-1292158
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Write
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-1296883
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Write
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-2321369
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Uncontrolled Recursion
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-2341553
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Write
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-2993560
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-Bounds
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-3041902
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Allocation of Resources Without Limits or Throttling
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-517892
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Uncontrolled Recursion
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-522743
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Allocation of Resources Without Limits or Throttling
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-523191
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Use After Free
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-524967
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-525795
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Write
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-527784
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Out-of-bounds Read
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-528223
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in binutils/binutils-common
  Description: Integer Overflow or Wraparound
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BINUTILS-529902
  Introduced through: gcc-defaults/gcc@4:10.2.1-1
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/libbinutils@2.35.2-2 > binutils/binutils-common@2.35.2-2
  From: gcc-defaults/gcc@4:10.2.1-1 > gcc-10@10.2.1-6 > binutils@2.35.2-2 > binutils/binutils-x86-64-linux-gnu@2.35.2-2 > binutils/binutils-common@2.35.2-2
  and 7 more...
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in bash
  Description: CVE-2022-3715
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-BASH-3112361
  Introduced through: bash@5.1-2+deb11u1
  From: bash@5.1-2+deb11u1
  Image layer: Introduced by your base image (rust:slim)

✗ Low severity vulnerability found in apt/libapt-pkg6.0
  Description: Improper Verification of Cryptographic Signature
  Info: https://snyk.io/vuln/SNYK-DEBIAN11-APT-522585
  Introduced through: apt/libapt-pkg6.0@2.2.4, apt@2.2.4
  From: apt/libapt-pkg6.0@2.2.4
  From: apt@2.2.4 > apt/libapt-pkg6.0@2.2.4
  From: apt@2.2.4
  Image layer: Introduced by your base image (rust:slim)



Package manager:   deb
Target file:       dockerfile
Project name:      docker-image|tp2devops
Docker image:      tp2devops
Platform:          linux/amd64
Base image:        rust:slim

Tested 129 dependencies for known vulnerabilities, found 63 vulnerabilities.

Recommendations for your base image (rust:slim) are not available.
See above for details and fixes on individual vulnerabilities
```
