# OBS Source Service `obs-service-elixir_mix_deps`
[![Factory build result](https://build.opensuse.org/projects/openSUSE:Factory/packages/obs-service-elixir_mix_deps/badge.svg?type=default)](https://build.opensuse.org/package/show/openSUSE:Factory/obs-service-elixir_mix_deps)
[![CI](https://github.com/openSUSE/obs-service-elixir_mix_deps/actions/workflows/rust.yml/badge.svg)](https://github.com/openSUSE/obs-service-elixir_mix_deps/actions/workflows/rust.yml)

## Overview

**obs-service-elixir_mix_deps** is a service designed to manage Elixir dependencies within the Open Build Service (OBS). This tool simplifies the process of handling the packaging of Elixir projects by automating dependency management, ensuring that your Elixir applications are maintained efficiently.

## Features

- **Automatic Dependency Resolution**: Automatically resolves and vendors dependencies specified in your Elixir projects.
- **Customizable Configurations**: Supports customizable settings to fit various project needs.

## Installation

To install this service (once it's at least shipped to Factory) just run this command:

```bash
sudo zypper in obs-service-elixir_mix_deps
```

## Usage
Just use the service inside the `_service` file as you would usually do with other stacks:

```xml
<services>
	<service name="tar_scm" mode="manual">
		<param name="url">https://github.com/elixir-lang/ex_doc</param>
		<param name="versionformat">@PARENT_TAG@</param>
		<param name="revision">v0.38.1</param>
		<param name="scm">git</param>
		<param name="changesgenerate">enable</param>
		<param name="versionrewrite-pattern">v(.+)</param>
		<param name="versionrewrite-replacement">\1</param>
	</service>
	<service name="set_version" mode="manual">
		<param name="basename">ex_doc</param>
	</service>
	<service name="recompress" mode="manual">
		<param name="compression">xz</param>
		<param name="file">*.tar</param>
	</service>
	<service name="elixir_mix_deps" mode="manual">
		<param name="subdir">ex_doc</param>
		<param name="archivename">vendor.tar.gz</param>
		<param name="compression">gz</param>
	</service>
</services>
```

The service will produce an archive with a `deps` directory inside, that contains all the dependencies needed for the project, ready to be used as a `Source` in your `.spec` file. 


```spec
Name:           ex_doc
Version:        0.38.1
Release:        0
Summary:        ExDoc produces HTML and online documentation for Elixir projects
License:        Apache-2.0 AND MIT
Group:          Development/Libraries/Other
URL:            https://github.com/elixir-lang/ex_doc
Source0:        %{name}-%{version}.tar.xz
Source1:        vendor.tar.gz
BuildRequires:  elixir >= 1.15
BuildRequires:  elixir-hex
Obsoletes:      elixir-ex_doc < %{version}
Provides:       elixir-ex_doc = %{version}
BuildRoot:      %{_tmppath}/%{name}-%{version}-build
# ex_doc package IS arch dependent
# See https://github.com/elixir-lang/elixir/issues/2785 for details
# BuildArch:      noarch

%description
ExDoc is a tool to generate documentation for your Elixir projects. In case you
are looking for documentation for Elixir itself, check out Elixir's website.

%prep
%autosetup -a1 -v

%build
export LANG=en_US.UTF-8
export MIX_ENV=prod
export MIX_PATH=%{elixir_libdir}/hex/ebin
%{__mix} escript.build

%install
sed -i -e '1s|/usr/bin/env escript|/usr/bin/escript|' ex_doc
install -D -m 0755 ex_doc %{buildroot}%{_bindir}/ex_doc

%files
%defattr(-,root,root)
%doc README.md CHANGELOG.md
%license LICENSE
%attr(0755,root,root) %{_bindir}/ex_doc

%changelog
```

The only very important caveat is, you have to set `elixir-hex` as a `BuildRequires` dependency and you also have to set the `MIX_PATH` environment variable to reference the operating system's Hex.

