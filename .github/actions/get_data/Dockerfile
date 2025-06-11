# action will be run in python3 container
FROM python:3@sha256:eec1b4e88e8762b4711b9f5fd69648b96ac04864e56993769cf50a9891a1d317
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
