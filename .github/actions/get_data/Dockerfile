# action will be run in python3 container
FROM python:3@sha256:884da97271696864c2eca77c6362b1c501196d6377115c81bb9dd8d538033ec3
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
