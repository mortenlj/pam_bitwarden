# action will be run in python3 container
FROM python:3@sha256:28f60ab75da2183870846130cead1f6af30162148d3238348f78f89cf6160b5d
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
