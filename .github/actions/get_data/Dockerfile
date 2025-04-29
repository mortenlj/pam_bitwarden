# action will be run in python3 container
FROM python:3@sha256:c33390eacee652aecb774f9606c263b4f76415bc83926a6777ede0f853c6bc19
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
