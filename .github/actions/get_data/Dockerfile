# action will be run in python3 container
FROM python:3@sha256:19c3e9658c3bab523c6ddb025464f3707cf3b1297100827bdb6afbf0c937d99f
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
