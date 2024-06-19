version: '3'
services:
%{ for service in jsondecode(services) ~}
  ${service.name}:
    image: ${service.image}
    %{~ if length(service.environment) > 0 ~}
    environment:
    %{~ for env in service.environment ~}
      - ${env.name}=${env.value}
    %{~ endfor ~}
    %{~ endif ~}
    %{~ if length(service.ports) > 0 ~}
    ports:
    %{~ for port in service.ports ~}
      - "${port.from}:${port.to}"
    %{~ endfor ~}
    %{~ endif ~}
    %{~ if length(service.networks) > 0 ~}
    networks:
    %{~ for network in service.networks ~}
      - ${network}
    %{~ endfor ~}
    %{~ endif ~}
    %{~ if length(service.volumes) > 0 ~}
    volumes:
    %{~ for volume in service.volumes ~}
      - ${volume}
    %{~ endfor ~}
    %{~ endif ~}
%{~ endfor ~}

%{~ if length(jsondecode(networks)) > 0 ~}
networks:
%{ for network, config in jsondecode(networks) ~}
  ${network}:
    %{ if config.driver != null }
    driver: ${config.driver}
    %{ endif }
%{~ endfor ~}
%{~ endif ~}

%{~ if length(jsondecode(volumes)) > 0 ~}
volumes:
%{~ for volume in jsondecode(volumes) ~}
  ${volume}:
%{~ endfor ~}
%{~ endif ~}
