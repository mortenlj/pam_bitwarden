# action will be run in python3 container
FROM python:3@sha256:9819e5616923079cc16af4a93d4be92c0c487c6e02fd9027220381f3e125d64a
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
