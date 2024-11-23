# OBS Source Service `obs-service-elixir_mix_deps`

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
        <param name="url">https://github.com/trento-project/web.git</param>
        <param name="scm">git</param>
        <param name="revision">00976f451ee68e1cdb3c2a433a880d90a9891a6d</param>
        <param name="exclude">.git</param>
        <param name="exclude">.github</param>
        <param name="exclude">assets/package-lock.json</param>
        <param name="extract">assets/package-lock.json</param>
        <param name="versionformat">2.4.0+git.dev4.1732176836.00976f451</param>
        <param name="filename">trento-web</param>
    </service>
    <service name="set_version" mode="manual">
        <param name="file">trento-web.spec</param>
    </service>
    <service name="recompress" mode="manual">
        <param name="file">*.tar</param>
        <param name="compression">gz</param>
    </service>
    <service name="elixir_mix_deps" mode="manual">
        <param name="subdir">web</param>
        <param name="archivename">vendor.tar.gz</param>
        <param name="compression">gz</param>
    </service>
</services>
```

The service will produce an archive with a `deps` directory inside, that contains all the dependencies needed for the project, ready to be used as a `Source` in your `.spec` file. 
