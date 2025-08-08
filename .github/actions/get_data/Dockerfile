# action will be run in python3 container
FROM python:3@sha256:68d0775234842868248bfe185eece56e725d3cb195f511a21233d0f564dee501
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
