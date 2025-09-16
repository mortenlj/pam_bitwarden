# action will be run in python3 container
FROM python:3@sha256:2deb0891ec3f643b1d342f04cc22154e6b6a76b41044791b537093fae00b6884
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
