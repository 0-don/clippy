Name:           clippy
Version:        {{VERSION}}
Release:        1%{?dist}
Summary:        {{DESCRIPTION}}
License:        MIT
URL:            https://github.com/0-don/clippy
Source0:        %{url}/releases/download/v%{version}/clippy-%{version}-1.x86_64.rpm

%description
Privacy focused clipboard manager with sync and encryption.
Supports text, images, files, HTML, and RTF content with
cloud sync via Google Drive, end-to-end encryption,
smart search, favorites, and customizable hotkeys.

%prep

%build

%install
rpm2cpio %{SOURCE0} | cpio -idmv -D %{buildroot}

%files
%{_bindir}/clippy
%{_datadir}/applications/clippy.desktop
%{_datadir}/icons/hicolor/*/apps/clippy.png
