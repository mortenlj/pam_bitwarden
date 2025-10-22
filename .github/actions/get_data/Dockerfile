# action will be run in python3 container
FROM python:3@sha256:671d8548cfc6ec789cbdd74c8585bd614864d1005970167350a2586be8db5650
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
