# action will be run in python3 container
FROM python:3@sha256:5b95b240f2db781f34a5da81e6e2301378495b3ab78d689df325c937be267e1c
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
