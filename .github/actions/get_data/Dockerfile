# action will be run in python3 container
FROM python:3@sha256:307a3e44b2a6c7d9c5fbb8c3a6c46c4c1535cde8eeb85a3ccb3513862f20aaad
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
