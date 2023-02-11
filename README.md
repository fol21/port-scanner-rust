# port-scanner-rust
Simple tests for port availability in an ip address.

## Usage
Reads a JSON file and tests the port states in local or not adresses.

````bash
port-scanner-rust path_to_json/adresses.json
````

The JSON file must have this format. *port-scanner-rust* accepts nested objects and treats then as sessions until an array of adresses is presented. The example above provides the accepted sintax for adress (ip and ports), including a port range with or without a custom step:

````json
{
    "Ip Adresses Test": {
        "Google Adresses": [
            "google.com:443",
            "google.com:80",
            "https://google.com",
            "https://google.com:[8001:8003]",
            "https://google.com:[9000:3:9004]"
        ],
        "Localhost Adresses": [
            "localhost:80",
            "localhost:8080",
            "127.0.0.1:443",
            "127.0.0.1:6379"
        ],
        "Azure Devops Adresses": [
            "radixeng.pkgs.visualstudio.com:443",
            "radixeng.visualstudio.com:443",
            "radixeng.vsblob.visualstudio.com:443",
            "radixeng.vsrm.visualstudio.com:443",
            "radixeng.vssps.visualstudio.com:443",
            "radixeng.vstmr.visualstudio.com:443",
            "*.blob.core.windows.net",
            "*.dev.azure.com:443",
            "*.vsassets.io:443",
            "*.vsblob.visualstudio.com:443",
            "*.vssps.visualstudio.com:443",
            "*.vstmr.visualstudio.com:443",
            "app.vssps.visualstudio.com:443",
            "dev.azure.com:443",
            "login.microsoftonline.com:443",
            "management.core.windows.net:443",
            "vstsagentpackage.azureedge.net:443"
        ]
    }
}

````

Output shows the port states in fashion presented bellow:

````bash
[Ip Adresses Test]:

[Google Adresses]:

[google.com:443]:
216.239.32.10:443 is closed
216.239.34.10:443 is closed
216.239.36.10:443 is closed
216.239.38.10:443 is closed
142.251.133.174:443 is bound
216.239.32.10:443 is closed
216.239.34.10:443 is closed
216.239.36.10:443 is closed
216.239.38.10:443 is closed
...
[google.com:443]:
142.251.133.174:443 is bound
216.239.32.10:443 is closed
216.239.34.10:443 is closed
216.239.36.10:443 is closed
216.239.38.10:443 is closed
216.239.32.10:443 is closed
216.239.34.10:443 is closed
216.239.36.10:443 is closed
216.239.38.10:443 is closed
[end]
...
[Localhost Adresses]:

[localhost:80]:
127.0.0.1:80 is closed
[end]
[localhost:8080]:
127.0.0.1:8080 is available
[end]
127.0.0.1:443 is closed
127.0.0.1:6379 is bound
...
[Azure Devops Adresses]:
...
[domain.vssps.visualstudio.com:443]:
13.107.42.18:443 is bound
40.90.4.7:443 is closed
64.4.48.7:443 is closed
13.107.24.7:443 is closed
13.107.160.7:443 is closed
[end]
...

````
